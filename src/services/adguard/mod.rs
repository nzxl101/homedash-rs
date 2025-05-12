use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

use crate::config::APICredsConfig;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct AdGuardStats {
    pub num_dns_queries: u32,
    pub num_blocked_filtering: u32,
    pub num_replaced_safebrowsing: u32,
    pub num_replaced_safesearch: u32,
    pub num_replaced_parental: u32,
    pub avg_processing_time: f32,
}

#[allow(deprecated)]
async fn req(endpoint: String, credentials: APICredsConfig) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/{}", &credentials.url, endpoint);

    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Basic {}",
                base64::encode(format!(
                    "{}:{}",
                    &credentials.username, &credentials.password
                ))
            ),
        )
        .send()
        .await?;

    Ok(response)
}

pub async fn get_adguard_stats(
    credentials: APICredsConfig,
) -> Result<AdGuardStats, Box<dyn Error>> {
    let response = req(String::from("control/stats"), credentials).await?;
    let data = response.json::<AdGuardStats>().await?;

    Ok(data)
}
