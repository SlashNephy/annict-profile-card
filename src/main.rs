use actix_web::{Responder, HttpServer, HttpResponse, App};
use actix_web::client::Client as HttpClient;
use actix_web::web::Path;
use graphql_client::{GraphQLQuery, Response};
use sailfish::TemplateOnce;
use serde::Deserialize;
use env_logger;
use log::*;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default="default_http_addr")]
    http_addr: String,
    annict_token: String
}

fn default_http_addr() -> String { "0.0.0.0:8080".to_string() }

fn load_config() -> Config {
    envy::from_env().unwrap_or_else(|_| panic!("failed to load env"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = load_config();
    info!("HTTP Server is listening for {}", config.http_addr);

    HttpServer::new(||
        App::new()
            .service(get_index)
            .service(get_watching)
    )
    .bind(config.http_addr)?
    .run()
    .await
}

#[actix_web::get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header("Location", "https://github.com/SlashNephy/annict-profile-card")
        .finish()
}

#[actix_web::get("/watching/{username}")]
async fn get_watching(Path(username): Path<String>) -> impl Responder {
    let data = perform_annict_query(get_user_query::Variables {
        username,
        state: get_user_query::StatusState::WATCHING,
        order_by: get_user_query::WorkOrder {
            direction: get_user_query::OrderDirection::DESC,
            field: get_user_query::WorkOrderField::WATCHERS_COUNT
        },
        seasons: vec!["2021-spring".to_string()]
    }).await;

    let svg = WatchingSvgTemplate {
        name: data.user.unwrap().name
    }
        .render_once()
        .unwrap_or_else(|e| panic!("failed to render svg: {}", e.to_string()));

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg)
}

#[derive(TemplateOnce)]
#[template(path = "watching.svg")]
struct WatchingSvgTemplate {
    name: String
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/GetUserQuery.graphql",
    response_derives = "Debug"
)]
struct GetUserQuery;

async fn perform_annict_query(variables: get_user_query::Variables) -> get_user_query::ResponseData {
    let request_body = GetUserQuery::build_query(variables);
    trace!("Request: {:#?}", serde_json::to_value(&request_body).unwrap());

    let config = load_config();
    let client = HttpClient::default();

    let mut res = client.post("https://api.annict.com/graphql")
        .bearer_auth(config.annict_token)
        .header("User-Agent", "annict-profile-card/0.0.1")
        .send_json(&request_body)
        .await
        .unwrap_or_else(|e| panic!("failed to request GraphQL query: {}", e.to_string()));

    let response_body: Response<get_user_query::ResponseData> = res.json()
        .await
        .unwrap_or_else(|e| panic!("failed to parse GraphQL response json: {}", e.to_string()));
    trace!("Response: {:#?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");

        for error in &errors {
            error!("{:?}", error);
        }
    }

    return response_body.data
        .unwrap_or_else(|| panic!("failed to access data"));
}
