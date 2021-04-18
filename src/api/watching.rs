use actix_web::HttpResponse;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Path, Query};
use graphql_client::GraphQLQuery;
use sailfish::TemplateOnce;
use serde::Deserialize;
use futures::future::try_join_all;
use log::*;

use super::common;
use watching_query::*;
use log::Level::Trace;

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
    expose_image_url: bool
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
    }).await.map_err(|e| ErrorInternalServerError(e))?;
    
    if log_enabled!(Trace) {
        trace!("Response: {:#?}", &data);
    }

    let user: WatchingQueryUser = data.user.unwrap();
    
    let mut avatar_uri = user.avatar_url.unwrap();
    if !query.expose_image_url {
        avatar_uri = common::encode_image(avatar_uri)
            .await
            .map_err(|e| ErrorInternalServerError(e))?;
    }

    let works: Vec<WatchingQueryUserWorksNodes> = user
        .works.unwrap()
        .nodes.unwrap()
        .into_iter()
        .filter_map(|x| x)
        .collect();
    
    let mut image_uris: Vec<String> = (&works).into_iter()
        .filter_map(|x| x.image.as_ref())
        .filter_map(|x| x.recommended_image_url.as_ref())
        .map(|x| x.to_owned())
        .take(3)
        .collect();
    if !query.expose_image_url {
        image_uris = try_join_all(
            image_uris.into_iter()
                .map(|x| common::encode_image(x))
        ).await.map_err(|e| ErrorInternalServerError(e))?;
    }
    
    let svg = WatchingSvgTemplate {
        name: user.name,
        username: user.username,
        avatar_uri,
        works,
        image_uris
    }
        .render_once()
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(
        HttpResponse::Ok()
            .content_type("image/svg+xml")
            .body(svg)
    )
}
