use reqwest::{Client, Response}; // Remove blocking
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::database::{connect_db, get_tmdb_id, insert_tmdb_id, Poster};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused, non_snake_case)]
pub struct TMDB3Movie {
    pub title: String,
    pub poster_path: String,
    pub id: u32,
}

async fn req(endpoint: String, api_key: String) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.themoviedb.org/3/{}", endpoint);
    let response = client
        .get(url)
        .query(&[("api_key", api_key)])
        .send()
        .await?;

    Ok(response)
}

pub async fn get_movie_entry(tmdb_id: u32, api_key: String) -> Result<TMDB3Movie, Box<dyn Error>> {
    let conn = connect_db()?;

    // Try to get from cache first
    if let Ok(cache) = get_tmdb_id(&conn, tmdb_id) {
        return Ok(serde_json::from_str(&cache.json_data)?);
    }

    // Fetch from API if not in cache
    let response = req(format!("movie/{}", tmdb_id), api_key).await?;
    let mut movie = response.json::<TMDB3Movie>().await?;

    // Update the poster path to include the base URL
    movie.poster_path = format!("https://image.tmdb.org/t/p/original/{}", movie.poster_path);

    // Cache the result
    insert_tmdb_id(
        &conn,
        &Poster {
            id: None,
            title: movie.title.to_owned(),
            image: movie.poster_path.to_owned(),
            tvdb_id: None,
            tmdb_id: Some(movie.id),
            json_data: serde_json::to_string(&movie)?,
        },
    )?;

    // Close db connection
    conn.close().unwrap();

    Ok(movie)
}
