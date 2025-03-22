use homedash_rs::system_metrics::{get_system_metrics, SystemMetrics};
use tuono_lib::axum::http::StatusCode;
use tuono_lib::axum::Json;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn metrics(_: Request) -> Result<Json<SystemMetrics>, StatusCode> {
    get_system_metrics()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
