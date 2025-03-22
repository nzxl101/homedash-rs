use homedash_rs::config::get_config;
use homedash_rs::services::tvdb::{get_series_entry, TVDBV4SeriesData};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::response::Result;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn get_tvdb_series_entry(req: Request) -> Result<Json<TVDBV4SeriesData>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !config.tvdb.enabled {
        return Err(StatusCode::NOT_FOUND);
    }

    let tvdb_id = req
        .params
        .get("id")
        .and_then(|id| id.parse::<u32>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let series_entry = get_series_entry(tvdb_id, config.tvdb)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(series_entry))
}
