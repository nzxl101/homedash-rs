use homedash_rs::config::get_config;
use serde::Serialize;
use tuono_lib::{Props, Request, Response, Type};

#[derive(Debug, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct WeatherData {
    lat: f64,
    long: f64,
}

#[derive(Debug, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct IndexData {
    username: String,
    weather: WeatherData,
    background: Option<String>,
}

#[tuono_lib::handler]
async fn index_data(_req: Request) -> Response {
    let config = get_config().unwrap();

    let weather_data = WeatherData {
        lat: config.clone().weather_location[0],
        long: config.clone().weather_location[1],
    };
    let index_data = IndexData {
        username: config.clone().username,
        weather: weather_data,
        background: config.clone().background_url,
    };

    Response::Props(Props::new(index_data))
}
