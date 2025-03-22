use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{
    config::APIOauthConfig,
    database::{connect_db, get_tvdb_id, insert_tvdb_id, Poster},
    oauth::{retrieve_oauth, OAuthFields},
};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct TVDBV4Series {
    pub name: String,
    pub image: String,
    pub id: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct TVDBV4SeriesData {
    pub data: TVDBV4Series,
}

async fn req(endpoint: String, credentials: APIOauthConfig) -> Result<Response, Box<dyn Error>> {
    let oauth = retrieve_oauth(
        &credentials,
        OAuthFields {
            request: Some(String::from("tvdb")),
        },
    )
    .await?;

    let client = Client::new();
    let url = format!("https://api4.thetvdb.com/v4/{}", endpoint);
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", oauth.token))
        .send()
        .await?;

    Ok(response)
}

pub async fn get_series_entry(
    tvdb_id: u32,
    credentials: APIOauthConfig,
) -> Result<TVDBV4SeriesData, Box<dyn Error>> {
    let conn = connect_db()?;

    // Try to get from cache first
    if let Ok(cache) = get_tvdb_id(&conn, tvdb_id) {
        return Ok(serde_json::from_str(&cache.json_data)?);
    }

    // Fetch from API if not in cache
    let response = req(format!("series/{}", tvdb_id), credentials).await?;
    let data = response.json::<TVDBV4SeriesData>().await?;
    let series = data.data.clone();

    // Cache the result
    insert_tvdb_id(
        &conn,
        &Poster {
            id: None,
            title: series.name,
            image: series.image,
            tvdb_id: Some(series.id),
            tmdb_id: None,
            json_data: serde_json::to_string(&data)?,
        },
    )?;

    // Close db connection
    conn.close().unwrap();

    Ok(data)
}
