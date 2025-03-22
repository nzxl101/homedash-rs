use reqwest::Client;
use serde::Serialize;
use serde_derive::Deserialize;
use std::{
    error::Error,
    time::{self, SystemTime},
};

use crate::config::{write_oauth_config, APIOauthConfig, OauthValues};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct OAuthFields {
    pub request: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
struct TvDbBody {
    token: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
struct TvDbResponse {
    status: String,
    data: TvDbBody,
}

pub async fn retrieve_oauth(
    credentials: &APIOauthConfig,
    fields: OAuthFields,
) -> Result<OauthValues, Box<dyn Error>> {
    let client = Client::new();
    let date_now = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if Some(String::from("tvdb")) == fields.request && credentials.enabled {
        let mut values = OauthValues {
            service: String::from("tvdb"),
            token: credentials.token.clone(),
            expires_in: credentials.expires_in.clone(),
        };

        if date_now > credentials.expires_in {
            println!("Refreshing tvdb token");

            let response = client
                .post("https://api4.thetvdb.com/v4/login")
                .json(&serde_json::json!({
                    "apikey": credentials.api_key,
                }))
                .send()
                .await
                .unwrap();

            if !response.status().is_success() {
                return Err(Box::from("Failed to refresh tvdb token"));
            }

            let token_info: TvDbResponse = response.json().await?;
            values.token = token_info.data.token;
            values.expires_in = date_now + 30 * 24 * 60 * 60;

            write_oauth_config(&values).unwrap();

            println!("Refreshed tvdb token");
        }

        return Ok(values);
    }

    // Can add more oauth service logins here in the future

    Err(Box::from("No oauth service requested"))
}
