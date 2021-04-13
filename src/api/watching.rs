use actix_web::{Responder, HttpResponse};
use actix_web::client::Client as HttpClient;
use actix_web::web::Path;
use graphql_client::{GraphQLQuery, Response};
use sailfish::TemplateOnce;
use log::*;

use crate::config;

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
    works: Vec<watching_query::WatchingQueryUserWorksNodes>,
    image_urls: Vec<String>
}

#[actix_web::get("/watching/{username}")]
pub async fn get_watching(Path(username): Path<String>) -> impl Responder {
    let data = perform_annict_query(watching_query::Variables {
        username,
        state: watching_query::StatusState::WATCHING,
        order_by: watching_query::WorkOrder {
            direction: watching_query::OrderDirection::DESC,
            field: watching_query::WorkOrderField::WATCHERS_COUNT
        },
        seasons: vec!["2021-spring".to_string()]
    }).await;

    let user: watching_query::WatchingQueryUser = data.user.unwrap();
    let works: Vec<watching_query::WatchingQueryUserWorksNodes> = user
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
        image_urls
    }
        .render_once()
        .unwrap_or_else(|e| panic!("failed to render svg: {}", e.to_string()));

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg)
}

async fn perform_annict_query(variables: watching_query::Variables) -> watching_query::ResponseData {
    let request_body = WatchingQuery::build_query(variables);
    trace!("Request: {:#?}", serde_json::to_value(&request_body).unwrap());

    let config = config::load();
    let client = HttpClient::default();

    let response_body: Response<watching_query::ResponseData> = client.post("https://api.annict.com/graphql")
        .bearer_auth(config.annict_token)
        .header("User-Agent", "annict-profile-card/0.0.1")
        .send_json(&request_body)
        .await
        .unwrap_or_else(|e| panic!("failed to request GraphQL query: {}", e.to_string()))
        .json()
        .await
        .unwrap_or_else(|e| panic!("failed to parse GraphQL response json: {}", e.to_string()));
    trace!("Response: {:#?}", response_body);

    if let Some(errors) = response_body.errors {
        let text = errors.into_iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join("\n");
        panic!("there are errors\n{}", text);
    }

    return response_body.data.unwrap();
}
