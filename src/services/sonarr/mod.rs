use reqwest::{Client, Response}; // Remove blocking
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct SonarrV3Series {
    pub title: String,
    pub tvdbId: Option<u32>,
    pub tmdbId: Option<u32>,
    pub seriesType: String,
    pub added: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct SonarrV3WantedMissing {
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

pub async fn get_series(
    base_url: String,
    api_key: String,
) -> Result<Vec<SonarrV3Series>, Box<dyn Error>> {
    let response = req(base_url, String::from("series"), api_key).await?;
    let data = response.json::<Vec<SonarrV3Series>>().await?;

    Ok(data)
}

pub async fn get_wanted_missing(
    base_url: String,
    api_key: String,
) -> Result<SonarrV3WantedMissing, Box<dyn Error>> {
    let response = req(base_url, String::from("wanted/missing"), api_key).await?;
    let data = response.json::<SonarrV3WantedMissing>().await?;

    Ok(data)
}
