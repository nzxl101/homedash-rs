use reqwest::{Client, Response}; // Remove blocking
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct RadarrV3Movies {
    pub title: String,
    pub imdbId: Option<String>,
    pub tmdbId: Option<u32>,
    pub added: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct RadarrV3WantedMissing {
    pub totalRecords: u32,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/v3/{}", base_url, endpoint);
    let response = client.get(url).query(&[("apikey", api_key)]).send().await?;

    Ok(response)
}

pub async fn get_movies(
    base_url: String,
    api_key: String,
) -> Result<Vec<RadarrV3Movies>, Box<dyn Error>> {
    let response = req(base_url, String::from("movie"), api_key).await?;
    let data = response.json::<Vec<RadarrV3Movies>>().await?;

    Ok(data)
}

pub async fn get_wanted_missing(
    base_url: String,
    api_key: String,
) -> Result<RadarrV3WantedMissing, Box<dyn Error>> {
    let response = req(base_url, String::from("wanted/missing"), api_key).await?;
    let data = response.json::<RadarrV3WantedMissing>().await?;

    Ok(data)
}
