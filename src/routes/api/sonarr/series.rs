use homedash_rs::config::get_config;
use homedash_rs::services::sonarr::{get_series, SonarrV3Series};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn series(_req: Request) -> Result<Json<Vec<SonarrV3Series>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.sonarr.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let series = get_series(config.sonarr.url, config.sonarr.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(series))
}
