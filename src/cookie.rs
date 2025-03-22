use reqwest::Client;
use serde::Serialize;
use serde_derive::Deserialize;
use std::error::Error;

use crate::config::{write_cookie_config, APICredsConfig, CookieValues};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct CookieFields {
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

pub async fn retrieve_cookie(
    credentials: &APICredsConfig,
    fields: CookieFields,
) -> Result<CookieValues, Box<dyn Error>> {
    let client = Client::new();

    if Some(String::from("qbittorrent")) == fields.request && credentials.enabled {
        let mut values = CookieValues {
            service: String::from("qbittorrent"),
            cookie: credentials.cookie.clone().unwrap_or_default(),
        };

        println!("Retrieving qbittorrent cookie");

        let url = format!("{}/api/v2/auth/login", credentials.url);
        let response = client
            .post(url)
            .form(&[
                ("username", &credentials.username),
                ("password", &credentials.password),
            ])
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            return Err(Box::from("Failed to retrieve qbittorrent cookie"));
        }
        // Get the cookie from the response headers
        if let Some(cookie_header) = response.headers().get("set-cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                // Extract the first part of the cookie (before the first semicolon)
                if let Some(cookie_value) = cookie_str.split(';').next() {
                    values.cookie = cookie_value.to_string();
                }
            }
        }

        write_cookie_config(&values).unwrap();

        println!("Retrieved qbittorrent cookie");

        return Ok(values);
    }

    // Can add more cookie service logins here in the future

    Err(Box::from("No cookie service requested"))
}
