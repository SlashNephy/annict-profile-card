use std::time::Duration;
use graphql_client::{GraphQLQuery, Response};
use log::*;
use chrono::{Local, Datelike};

use crate::config;
use crate::api::error::ApiError;
use log::Level::Trace;

const ANNICT_GRAPHQL_ENDPOINT: &str = "https://api.annict.com/graphql";
const USER_AGENT: &str = "annict-profile-card (+https://github.com/SlashNephy/annict-profile-card)";

pub async fn perform_query<Q: GraphQLQuery + 'static>(variables: Q::Variables) -> Result<Q::ResponseData, ApiError> {
    let request_body = Q::build_query(variables);

    if log_enabled!(Trace) {
        trace!("Request: {:#?}", serde_json::to_value(&request_body).unwrap());
    }

    let config = config::load();
    let client = awc::Client::default();

    let mut response_body = client.post(ANNICT_GRAPHQL_ENDPOINT)
        .bearer_auth(config.annict_token)
        .append_header(("User-Agent", USER_AGENT))
        .timeout(Duration::from_secs(15))
        .send_json(&request_body)
        .await
        .map_err(|e| ApiError::AnnictGraphQLRequestError(e))?;

    if log_enabled!(Trace) {
        trace!("Response Header: {:#?}", &response_body);
    }

    let response: Response<Q::ResponseData> = response_body.json()
        .await
        .map_err(|e| ApiError::AnnictGraphQLResponseParseError(e))?;

    if let Some(errors) = response.errors {
        let text = errors.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        return Err(
            ApiError::AnnictGraphQLResponseError(text)
        )
    }

    Ok(
        response.data.unwrap()
    )
}

pub async fn encode_image(url: String) -> Result<String, ApiError> {
    let client = awc::Client::default();
    let image = client.get(url)
        .append_header(("User-Agent", USER_AGENT))
        .timeout(Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| ApiError::ImageRequestError(e))?
        .body()
        .limit(3_145_728) // 3 MB
        .await
        .map_err(|e| ApiError::ImageReadBodyError(e))?;

    let data = base64::encode(image);
    Ok(
        format!("data:image/png;base64,{}", data)
    )
}

pub fn get_current_season() -> String {
    let now = Local::today();
    let year = now.year();
    let season = match now.month() {
        1..=3 => "winter",
        4..=6 => "spring",
        7..=9 => "summer",
        10..=12 => "autumn",
        _ => panic!("Invalid month")
    };

    format!("{}-{}", year, season)
}
