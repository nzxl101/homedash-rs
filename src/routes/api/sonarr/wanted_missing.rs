use homedash_rs::config::get_config;
use homedash_rs::services::sonarr::{get_wanted_missing, SonarrV3WantedMissing};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn wanted_missing(_req: Request) -> Result<Json<SonarrV3WantedMissing>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.sonarr.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let missing = get_wanted_missing(config.sonarr.url, config.sonarr.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(missing))
}
