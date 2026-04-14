use homedash_rs::config::get_config;
use homedash_rs::services::gluetun::{get_vpn_public_ip, GluetunV1VPNPublicIP};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn vpn_public_ip(_req: Request) -> Result<Json<GluetunV1VPNPublicIP>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.gluetun.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let public_ip = get_vpn_public_ip(config.gluetun.url, config.gluetun.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(public_ip))
}
