use homedash_rs::database::{connect_db, update_app_preferences, Preference};
use serde::Deserialize;
use tuono_lib::axum::http::StatusCode;
use tuono_lib::Request;

#[derive(Deserialize)]
struct Body {
    apps: Vec<Preference>,
}

#[tuono_lib::api(POST)]
pub async fn save(req: Request) -> Result<String, StatusCode> {
    let body: Body = req.body().map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut conn = connect_db().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match update_app_preferences(&mut conn, body.apps) {
        Ok(_) => {
            conn.close()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(String::from("OK"))
        }
        Err(_) => {
            conn.close()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
