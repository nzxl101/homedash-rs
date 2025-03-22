use reqwest::{Client, Response}; // Remove blocking
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct OverseerrV1Media {
    tvdbId: Option<u32>,
    tmdbId: Option<u32>,
    mediaType: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct OverseerrV1RequestedBy {
    plexUsername: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct OverseerrV1Request {
    media: OverseerrV1Media,
    requestedBy: OverseerrV1RequestedBy,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct OverseerrV1Requests {
    results: Vec<OverseerrV1Request>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct OverseerrV1RequestsCount {
    available: u32,
    processing: u32,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/v1/{}", base_url, endpoint);
    let response = client.get(url).header("X-Api-Key", api_key).send().await?;

    Ok(response)
}

pub async fn get_requests(
    base_url: String,
    api_key: String,
) -> Result<OverseerrV1Requests, Box<dyn Error>> {
    let response = req(base_url, String::from("request"), api_key).await?;
    let data = response.json::<OverseerrV1Requests>().await?;

    Ok(data)
}

pub async fn get_requests_count(
    base_url: String,
    api_key: String,
) -> Result<OverseerrV1RequestsCount, Box<dyn Error>> {
    let response = req(base_url, String::from("request/count"), api_key).await?;
    let data = response.json::<OverseerrV1RequestsCount>().await?;

    Ok(data)
}
