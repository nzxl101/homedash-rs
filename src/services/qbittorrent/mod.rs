use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{
    config::APICredsConfig,
    cookie::{retrieve_cookie, CookieFields},
};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct QBitV2Torrent {
    pub name: String,
    pub size: u64,
    pub progress: f32,
    pub state: String,
    pub num_seeds: u32,
    pub num_leechs: u32,
    pub dlspeed: u64,
    pub upspeed: u64,
    pub eta: u32,
    pub ratio: f32,
}

async fn req(endpoint: String, credentials: APICredsConfig) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/v2/{}", &credentials.url, endpoint);

    // Try with cached cookie first
    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header("Cookie", credentials.cookie.clone().unwrap_or_default())
        .send()
        .await?;

    // Only get new cookie if cached one fails
    if response.status() == 403 {
        let cookie_result = retrieve_cookie(
            &credentials,
            CookieFields {
                request: Some(String::from("qbittorrent")),
            },
        )
        .await?;

        return Ok(client
            .get(url)
            .header("Content-Type", "application/json")
            .header("Cookie", cookie_result.cookie)
            .send()
            .await?);
    }

    Ok(response)
}

pub async fn get_torrents(
    credentials: APICredsConfig,
) -> Result<Vec<QBitV2Torrent>, Box<dyn Error>> {
    let response = req(String::from("torrents/info"), credentials).await?;
    let data = response.json::<Vec<QBitV2Torrent>>().await?;

    Ok(data)
}
