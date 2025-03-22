use homedash_rs::config::get_config;
use homedash_rs::services::tautulli::{get_stream_sessions, TautulliV2Sessions};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn sessions(_req: Request) -> Result<Json<TautulliV2Sessions>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.tautulli.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let sessions = get_stream_sessions(config.tautulli.url, config.tautulli.api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sessions))
}
