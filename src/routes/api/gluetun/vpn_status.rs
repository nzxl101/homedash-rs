use homedash_rs::config::get_config;
use homedash_rs::services::gluetun::{get_vpn_status, GluetunV1VPNStatus};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn vpn_status(_req: Request) -> Result<Json<GluetunV1VPNStatus>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.gluetun.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let vpn_status = get_vpn_status(config.gluetun.url, config.gluetun.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vpn_status))
}
