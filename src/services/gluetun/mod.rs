use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct GluetunV1VPNPublicIP {
    pub public_ip: String,
    pub country: Option<String>,
    pub city: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct GluetunV1VPNStatus {
    pub status: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct GluetunV1VPNOutcome {
    pub outcome: String,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
    method: String,
    data: Option<String>,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/v1/{}", base_url, endpoint);

    if &method == "put" && !data.is_none() {
        let response = client
            .put(url)
            .header("Content-Type", "application/json")
            .header("X-API-Key", api_key)
            .body(data.unwrap())
            .send()
            .await?;

        return Ok(response);
    }

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("X-API-Key", api_key)
        .send()
        .await?;

    Ok(response)
}

pub async fn get_vpn_public_ip(
    base_url: String,
    api_key: String,
) -> Result<GluetunV1VPNPublicIP, Box<dyn Error>> {
    let response = req(
        base_url,
        String::from("publicip/ip"),
        api_key,
        String::from("get"),
        None,
    )
    .await?;
    let data = response.json::<GluetunV1VPNPublicIP>().await?;

    Ok(data)
}

pub async fn get_vpn_status(
    base_url: String,
    api_key: String,
) -> Result<GluetunV1VPNStatus, Box<dyn Error>> {
    let response = req(
        base_url,
        String::from("vpn/status"),
        api_key,
        String::from("get"),
        None,
    )
    .await?;
    let data = response.json::<GluetunV1VPNStatus>().await?;

    Ok(data)
}

pub async fn put_vpn_reconnect(
    base_url: String,
    api_key: String,
) -> Result<Option<GluetunV1VPNOutcome>, Box<dyn Error>> {
    let response = req(
        base_url.clone(),
        String::from("vpn/status"),
        api_key.clone(),
        String::from("put"),
        Some(String::from("{\"status\": \"stopped\"}")),
    )
    .await?;
    let data = response.json::<GluetunV1VPNOutcome>().await?;

    if &data.outcome == "stopped" {
        let response = req(
            base_url.clone(),
            String::from("vpn/status"),
            api_key.clone(),
            String::from("put"),
            Some(String::from("{\"status\": \"running\"}")),
        )
        .await?;
        let data = response.json::<GluetunV1VPNOutcome>().await?;

        return Ok(Some(data));
    }

    // Failed to reconnect
    Ok(None)
}
