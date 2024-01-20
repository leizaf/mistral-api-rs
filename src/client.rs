use reqwest::{Client, Request, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

pub trait Endpoint {
    type Response: DeserializeOwned;
    fn request(&self, client: &Client) -> Request;
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

pub type QueryResult<T> = Result<T, ApiError>;

pub trait Query: Endpoint {
    async fn query(&self, client: &Client) -> QueryResult<Self::Response>;
}

impl<E> Query for E
where
    E: Endpoint,
{
    async fn query(&self, client: &Client) -> QueryResult<Self::Response> {
        let rsp = client.execute(self.request(client)).await?;
        let status = rsp.status();
        let raw_body = rsp.bytes().await?;
        let val = serde_json::from_slice::<Value>(raw_body.as_ref())
            .map_err(|_| ApiError::Server(status, raw_body.to_vec()))?;
        if !status.is_success() {
            return Err(ApiError::Mistral(val));
        }
        serde_json::from_value(val).map_err(ApiError::Json)
    }
}
