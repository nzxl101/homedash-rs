use homedash_rs::config::get_config;
use homedash_rs::services::proxmox::{get_proxmox_data, ProxmoxV2Data};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn node(_req: Request) -> Result<Json<Vec<ProxmoxV2Data>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.proxmox.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let data = get_proxmox_data(config.proxmox)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}
