use homedash_rs::config::get_config;
use homedash_rs::services::qbittorrent::{get_torrents, QBitV2Torrent};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn torrents(_req: Request) -> Result<Json<Vec<QBitV2Torrent>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.qbittorrent.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let torrents = get_torrents(config.qbittorrent)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(torrents))
}
