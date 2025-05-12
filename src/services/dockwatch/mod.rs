use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct Status {
    pub total: u32,
    pub running: u32,
    pub stopped: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct Health {
    pub healthy: u32,
    pub unhealthy: u32,
    pub unknown: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct Updates {
    pub uptodate: u32,
    pub outdated: u32,
    pub unchecked: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct Usage {
    pub cpu: f32,
    pub memory: f32,
    pub netIO: u64,
    pub disk: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct DockwatchStats {
    pub status: Status,
    pub health: Health,
    pub updates: Updates,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct DockwatchStatsResponse {
    pub response: DockwatchStats,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/{}", base_url, endpoint);
    let response = client.get(url).query(&[("apikey", api_key)]).send().await?;

    Ok(response)
}

pub async fn get_dockwatch_stats(
    base_url: String,
    api_key: String,
) -> Result<DockwatchStatsResponse, Box<dyn Error>> {
    let response = req(base_url, String::from("stats/overview"), api_key).await?;
    let data = response.json::<DockwatchStatsResponse>().await?;

    Ok(data)
}
