use homedash_rs::config::get_config;
use homedash_rs::services::plex::{get_library_media_count, PlexLibraryCount};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn library_count(_req: Request) -> Result<Json<PlexLibraryCount>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.plex.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let count = get_library_media_count(config.plex.url, config.plex.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(count))
}
