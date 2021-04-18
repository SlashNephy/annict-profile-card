use actix_web::client::{SendRequestError, JsonPayloadError, PayloadError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("failed to request GraphQL query: {0}")]
    AnnictGraphQLRequestError(SendRequestError),
    #[error("failed to parse GraphQL response: {0}")]
    AnnictGraphQLResponseParseError(JsonPayloadError),
    #[error("an error returned from GraphQL server: {0}")]
    AnnictGraphQLResponseError(String),
    
    #[error("failed to get image: {0}")]
    ImageRequestError(SendRequestError),
    #[error("failed to read image body: {0}")]
    ImageReadBodyError(PayloadError)
}
