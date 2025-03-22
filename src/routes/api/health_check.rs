use tuono_lib::axum::http::StatusCode;
use tuono_lib::Request;

#[tuono_lib::api(GET)]
pub async fn health_check(_req: Request) -> StatusCode {
    StatusCode::OK
}
