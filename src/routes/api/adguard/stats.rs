use homedash_rs::config::get_config;
use homedash_rs::services::adguard::{get_adguard_stats, AdGuardStats};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn stats(_req: Request) -> Result<Json<AdGuardStats>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.adguard.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let adguard_stats = get_adguard_stats(config.adguard)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(adguard_stats))
}
