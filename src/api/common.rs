use actix_web::client::Client as HttpClient;
use graphql_client::{GraphQLQuery, Response};
use log::*;

use crate::config;

pub const CURRENT_SEASON: &str = "2021-spring";

const ANNICT_GRAPHQL_ENDPOINT: &str = "https://api.annict.com/graphql";
const USER_AGENT: &str = "annict-profile-card/0.0.1";

pub async fn perform_query<Q: GraphQLQuery + 'static>(variables: Q::Variables) -> Q::ResponseData {
    let request_body = Q::build_query(variables);
    trace!("Request: {:#?}", serde_json::to_value(&request_body).unwrap());

    let config = config::load();
    let client = HttpClient::default();

    let mut response_body = client.post(ANNICT_GRAPHQL_ENDPOINT)
        .bearer_auth(config.annict_token)
        .header("User-Agent", USER_AGENT)
        .send_json(&request_body)
        .await
        .unwrap_or_else(|e| panic!("failed to request GraphQL query: {}", e.to_string()));
    trace!("Response: {:#?}", &response_body);

    let response: Response<Q::ResponseData> = response_body.json()
        .await
        .unwrap_or_else(|e| panic!("failed to parse GraphQL response json: {}", e.to_string()));

    if let Some(errors) = response.errors {
        let text = errors.into_iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join("\n");
        panic!("there are errors\n{}", text);
    }

    return response.data.unwrap();
}
