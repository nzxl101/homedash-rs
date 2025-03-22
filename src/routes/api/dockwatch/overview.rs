use homedash_rs::config::get_config;
use homedash_rs::services::dockwatch::{get_dockwatch_stats, DockwatchStatsResponse};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn overview(_req: Request) -> Result<Json<DockwatchStatsResponse>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.dockwatch.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let dockwatch_stats = get_dockwatch_stats(config.dockwatch.url, config.dockwatch.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(dockwatch_stats))
}
