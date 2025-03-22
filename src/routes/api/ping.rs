use homedash_rs::config::get_config;
use homedash_rs::database::{connect_db, get_ping_data, Ping};
use homedash_rs::ping::ping_all_urls;
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn ping(_req: Request) -> Result<Json<Vec<Ping>>, StatusCode> {
    let config = get_config().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let conn = connect_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    ping_all_urls(&config, &conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = get_ping_data(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Close db connection
    conn.close()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(data))
}
