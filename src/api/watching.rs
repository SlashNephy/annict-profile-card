use actix_web::HttpResponse;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Path, Query};
use graphql_client::GraphQLQuery;
use sailfish::TemplateOnce;
use serde::Deserialize;
use futures::future::join_all;
use log::*;

use super::common;
use watching_query::*;
use log::Level::Trace;
use crate::api::watching::SortKey::SatisfactionRate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema.graphql",
    query_path = "src/api/watching.graphql",
    response_derives = "Debug"
)]
struct WatchingQuery;

#[derive(Deserialize)]
pub struct WatchingParameter {
    #[serde(default)]
    sort: SortKey,
    #[serde(default)]
    expose_image_url: bool
}

#[derive(Deserialize, PartialEq)]
enum SortKey {
    #[serde(alias = "watcher")]
    WatchersCount,
    #[serde(alias = "satisfaction")]
    SatisfactionRate
}

impl Default for SortKey {
    fn default() -> Self {
        SatisfactionRate
    }
}

#[derive(TemplateOnce)]
#[template(path = "watching.svg")]
struct WatchingSvgTemplate {
    name: String,
    username: String,
    avatar_uri: String,
    works: Vec<WatchingQueryUserWorksNodes>,
    image_uris: Vec<String>
}

#[actix_web::get("/watching/{username}")]
pub async fn get_watching(Path(username): Path<String>, query: Query<WatchingParameter>) -> actix_web::Result<HttpResponse> {
    let data = common::perform_query::<WatchingQuery>(Variables {
        username,
        state: StatusState::WATCHING,
        order_by: WorkOrder {
            direction: OrderDirection::DESC,
            field: WorkOrderField::WATCHERS_COUNT
        },
        seasons: vec![String::from(common::CURRENT_SEASON)]
    }).await.map_err(|e| {
        ErrorInternalServerError(e)
    })?;
    
    if log_enabled!(Trace) {
        trace!("Response: {:#?}", &data);
    }

    // ユーザオブジェクト
    let user: WatchingQueryUser = data.user.unwrap();

    // プロフィール画像
    let original_avatar_url = user.avatar_url.unwrap();
    let avatar_uri = match query.expose_image_url {
        true => original_avatar_url,
        false => {
            // base64 エンコードする
            match common::encode_image(original_avatar_url).await {
                Ok(uri) => uri,
                _ => String::from("data:image/png;base64,")
            }
        }
    };

    // 作品のベクトル
    let mut works: Vec<WatchingQueryUserWorksNodes> = user
        .works.unwrap()
        .nodes.unwrap()
        .into_iter()
        .filter_map(|x| x)
        .collect();
    // 満足度の降順でソート
    if query.sort == SortKey::SatisfactionRate {
        works.sort_unstable_by(|x, y| {
            let rate_x: f64 = x.satisfaction_rate.unwrap_or(0.0);
            let rate_y: f64 = y.satisfaction_rate.unwrap_or(0.0);
            rate_y.partial_cmp(&rate_x).unwrap()
        });
    }
    
    // 作品のアイキャッチ画像のベクトル
    let original_image_uris: Vec<String> = (&works).into_iter()
        .filter_map(|x| x.image.as_ref())
        .filter_map(|x| x.recommended_image_url.as_ref())
        .map(|x| x.to_owned())
        .take(3)
        .collect();
    let image_uris = match query.expose_image_url {
        true => original_image_uris,
        false => {
            // 並列に base64 エンコードする
            let job = join_all(
                original_image_uris
                    .into_iter()
                    .map(|x| {
                        common::encode_image(x)
                    })
            );
    
            job.await
                .into_iter()
                .map(|x| {
                    match x {
                        Ok(uri) => uri,
                        // 失敗したら空画像に差し替える
                        Err(_) => String::from("data:image/png;base64,")
                    }
                })
                .collect()
        }
    };
    
    let svg = WatchingSvgTemplate {
        name: user.name,
        username: user.username,
        avatar_uri,
        works,
        image_uris
    }
        .render_once()
        .map_err(|e| {
            ErrorInternalServerError(e)
        })?;

    Ok(
        HttpResponse::Ok()
            .content_type("image/svg+xml")
            .body(svg)
    )
}
