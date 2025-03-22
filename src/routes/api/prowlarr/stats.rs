use homedash_rs::config::get_config;
use homedash_rs::services::prowlarr::{get_indexer_stats, ProwlarrV1IndexerStats};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn indexer_stats(_req: Request) -> Result<Json<ProwlarrV1IndexerStats>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.prowlarr.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let stats = get_indexer_stats(config.prowlarr.url, config.prowlarr.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stats))
}
