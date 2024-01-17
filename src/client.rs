use reqwest::{Client, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

pub trait Endpoint {
    fn request(&self, client: Client) -> Request;
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Client error: {0}")]
    Client(#[from] reqwest::Error),
    #[error("Mistral internal server error: {0}")]
    Server(StatusCode, Vec<u8>),
    #[error("Mistral error: {0}")]
    Mistral(Value),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

impl ApiError {
    fn server_error(status: StatusCode, body: Vec<u8>) -> Self {
        Self::Server(status, body)
    }
}

pub type QueryResult<T> = Result<T, ApiError>;

pub trait Query<T>: Endpoint
where
    T: DeserializeOwned,
{
    fn query(&self, client: Client) -> QueryResult<T>;

    async fn query_async(&self, client: Client) -> QueryResult<T> {
        let rsp = client.execute(self.request(client)).await?;
        let status = rsp.status();
        let raw_body = rsp.bytes().await?;
        let val: Value = serde_json::from_slice(raw_body.as_ref())?
            .map_err(|_| ApiError::server_error(status, raw_body.into_vec()))?;
        if !status.is_success() {
            return Err(ApiError::Mistral(val));
        }
        serde_json::from_value(val).map_err(ApiError::Json)
    }
}
