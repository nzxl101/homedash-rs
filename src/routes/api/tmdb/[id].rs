use homedash_rs::config::get_config;
use homedash_rs::services::tmdb::{get_movie_entry, TMDB3Movie};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn get_tmdb_movie_entry(req: Request) -> Result<Json<TMDB3Movie>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.tmdb.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let tmdb_id = req
        .params
        .get("id")
        .and_then(|id| id.parse::<u32>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let movie_entry = get_movie_entry(tmdb_id, config.tmdb.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(movie_entry))
}
