use homedash_rs::config::get_config;
use homedash_rs::services::radarr::{get_movies, RadarrV3Movies};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn movies(_req: Request) -> Result<Json<Vec<RadarrV3Movies>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.radarr.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let movies = get_movies(config.radarr.url, config.radarr.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(movies))
}
