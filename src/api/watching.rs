use actix_web::{HttpResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::Path;
use graphql_client::GraphQLQuery;
use sailfish::TemplateOnce;

use super::common;
use watching_query::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema.graphql",
    query_path = "src/api/watching.graphql",
    response_derives = "Debug"
)]
struct WatchingQuery;

#[derive(TemplateOnce)]
#[template(path = "watching.svg")]
struct WatchingSvgTemplate {
    name: String,
    username: String,
    avatar_url: String,
    works: Vec<WatchingQueryUserWorksNodes>,
    image_urls: Vec<String>,
    render_as_data_uri: bool
}

#[actix_web::get("/watching/{username}")]
pub async fn get_watching(Path(username): Path<String>) -> actix_web::Result<HttpResponse> {
    let data = common::perform_query::<WatchingQuery>(Variables {
        username,
        state: StatusState::WATCHING,
        order_by: WorkOrder {
            direction: OrderDirection::DESC,
            field: WorkOrderField::WATCHERS_COUNT
        },
        seasons: vec![String::from(common::CURRENT_SEASON)]
    }).await;

    let user: WatchingQueryUser = data.user.unwrap();
    let works: Vec<WatchingQueryUserWorksNodes> = user
        .works.unwrap()
        .nodes.unwrap()
        .into_iter()
        .filter_map(|x| x)
        .collect();
    let image_urls = (&works).into_iter()
        .filter_map(|x| x.image.as_ref())
        .filter_map(|x| x.recommended_image_url.as_ref())
        .map(|x| x.to_owned())
        .take(3)
        .collect();
    let svg = WatchingSvgTemplate {
        name: user.name,
        username: user.username,
        avatar_url: user.avatar_url.unwrap(),
        works,
        image_urls,
        render_as_data_uri: false
    }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(
        HttpResponse::Ok()
            .content_type("image/svg+xml")
            .body(svg)
    )
}
