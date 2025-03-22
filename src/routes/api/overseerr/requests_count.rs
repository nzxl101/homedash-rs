use homedash_rs::config::get_config;
use homedash_rs::services::overseerr::{get_requests_count, OverseerrV1RequestsCount};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn requests(_req: Request) -> Result<Json<OverseerrV1RequestsCount>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.overseerr.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let data = get_requests_count(config.overseerr.url, config.overseerr.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}
