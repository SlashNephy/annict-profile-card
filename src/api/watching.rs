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
use actix_web::http::header::{CacheControl, CacheDirective};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema.graphql",
    query_path = "src/api/watching.graphql",
    response_derives = "Debug"
)]
struct WatchingQuery;

#[derive(Deserialize, Debug, Clone)]
pub struct WatchingParameter {
    #[serde(default)]
    season: Option<String>,
    #[serde(default = "default_bg_color")]
    bg_color: String,
    #[serde(default = "default_header_color")]
    header_color: String,
    #[serde(default = "default_text_color")]
    text_color: String,
    #[serde(default = "default_icon_color")]
    icon_color: String,
    #[serde(default = "default_title_color")]
    title_color: String,
    #[serde(default = "default_limit_works")]
    limit_works: usize,
    #[serde(default = "default_limit_images")]
    limit_images: usize,
    #[serde(default)]
    sort: SortKey,
    #[serde(default)]
    order: SortOrder,
    #[serde(default)]
    expose_image_url: bool
}

fn default_bg_color() -> String { String::from("1a1b27") }
fn default_header_color() -> String { String::from("70a5fd") }
fn default_text_color() -> String { String::from("d6e3e1") }
fn default_icon_color() -> String { String::from("bf91f3") }
fn default_title_color() -> String { String::from("38bdae") }
fn default_limit_works() -> usize { 10 }
fn default_limit_images() -> usize { 3 }

#[derive(Deserialize, PartialEq, Debug, Clone)]
enum SortKey {
    #[serde(alias = "watcher")]
    WatchersCount,
    #[serde(alias = "satisfaction")]
    SatisfactionRate
}

impl Default for SortKey {
    fn default() -> Self {
        SortKey::SatisfactionRate
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
enum SortOrder {
    #[serde(alias = "desc")]
    Descending,
    #[serde(alias = "asc")]
    Ascending
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Descending
    }
}

#[derive(TemplateOnce)]
#[template(path = "watching.svg")]
struct WatchingSvgTemplate {
    query: Query<WatchingParameter>,
    name: String,
    username: String,
    avatar_uri: String,
    works: Vec<WatchingQueryUserWorksNodes>,
    works_count: usize,
    image_uris: Vec<String>
}

#[actix_web::get("/watching/{username}")]
pub async fn get_watching(path: Path<String>, query: Query<WatchingParameter>) -> actix_web::Result<HttpResponse> {
    let username = path.into_inner();
    let data = common::perform_query::<WatchingQuery>(Variables {
        username,
        state: StatusState::WATCHING,
        order_by: WorkOrder {
            direction: match query.order.clone() {
                SortOrder::Ascending => OrderDirection::ASC,
                SortOrder::Descending => OrderDirection::DESC
            },
            field: WorkOrderField::WATCHERS_COUNT
        },
        seasons: match query.season.as_deref() {
            Some("all") => vec![],
            Some(value) => vec![String::from(value)],
            None => vec![common::get_current_season()]
        }
    }).await.map_err(|e| {
        ErrorInternalServerError(e)
    })?;

    if log_enabled!(Trace) {
        trace!("Query: {:#?}", &query);
        trace!("Response: {:#?}", &data);
    }

    // ???????????????????????????
    let user: WatchingQueryUser = match data.user {
        Some(user) => user,
        None => return Ok(
            HttpResponse::NotFound().finish()
        )
    };

    // ????????????????????????
    let original_avatar_url = user.avatar_url.unwrap();
    let avatar_uri = match query.expose_image_url {
        true => original_avatar_url,
        false => {
            // base64 ?????????????????????
            match common::encode_image(original_avatar_url).await {
                Ok(uri) => uri,
                Err(e) => {
                    warn!("An error occurred while encode_image: {:#?}", e);
                    String::from("data:image/png;base64,")
                }
            }
        }
    };

    // ?????????????????????
    let original_works: Vec<WatchingQueryUserWorksNodes> = user
        .works.unwrap()
        .nodes.unwrap()
        .into_iter()
        .filter_map(|x| x)
        .collect();
    let works_count = original_works.len();
    let mut works: Vec<WatchingQueryUserWorksNodes> = original_works;
    // ??????????????????????????????
    if query.sort == SortKey::SatisfactionRate {
        works.sort_unstable_by(|x, y| {
            let rate_x: f64 = x.satisfaction_rate.unwrap_or(0.0);
            let rate_y: f64 = y.satisfaction_rate.unwrap_or(0.0);

            match &query.order {
                SortOrder::Ascending => rate_x.partial_cmp(&rate_y).unwrap(),
                SortOrder::Descending => rate_y.partial_cmp(&rate_x).unwrap()
            }
        });
    }
    // limit_works ????????????
    works = works.into_iter()
        .take(query.limit_works)
        .collect();

    // ????????????????????????????????????????????????
    let original_image_uris: Vec<String> = (&works).into_iter()
        .filter_map(|x| x.image.as_ref())
        .filter_map(|x| x.recommended_image_url.as_ref())
        .map(|x| x.to_owned())
        .take(query.limit_images)
        .collect();
    let image_uris = match query.expose_image_url {
        true => original_image_uris,
        false => {
            // ????????? base64 ?????????????????????
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
                        // ??????????????????????????????????????????
                        Err(e) => {
                            warn!("An error occurred while encode_image: {:#?}", e);
                            String::from("data:image/png;base64,")
                        }
                    }
                })
                .collect()
        }
    };

    let svg = WatchingSvgTemplate {
        query,
        name: user.name,
        username: user.username,
        avatar_uri,
        works,
        works_count,
        image_uris
    }
        .render_once()
        .map_err(|e| {
            ErrorInternalServerError(e)
        })?;

    Ok(
        HttpResponse::Ok()
            .content_type("image/svg+xml")
            .append_header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::MaxAge(7200)
            ]))
            .body(svg)
    )
}
