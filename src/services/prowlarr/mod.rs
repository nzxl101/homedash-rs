use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProwlarrV1Hosts {
    pub host: String,
    pub numberOfQueries: u32,
    pub numberOfGrabs: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct ProwlarrV1IndexerStats {
    pub hosts: Vec<ProwlarrV1Hosts>,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/v1/{}", base_url, endpoint);
    let response = client.get(url).query(&[("apikey", api_key)]).send().await?;

    Ok(response)
}

pub async fn get_indexer_stats(
    base_url: String,
    api_key: String,
) -> Result<ProwlarrV1IndexerStats, Box<dyn Error>> {
    let response = req(base_url, String::from("indexerstats"), api_key).await?;
    let data = response.json::<ProwlarrV1IndexerStats>().await?;

    Ok(data)
}
