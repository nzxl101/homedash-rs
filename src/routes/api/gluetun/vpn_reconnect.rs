use homedash_rs::config::get_config;
use homedash_rs::services::gluetun::{put_vpn_reconnect, GluetunV1VPNOutcome};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn vpn_reconnect(_req: Request) -> Result<Json<Option<GluetunV1VPNOutcome>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.gluetun.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let vpn_outcome = put_vpn_reconnect(config.gluetun.url, config.gluetun.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vpn_outcome))
}
