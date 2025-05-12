use std::process;
use std::time::Instant;

use homedash_rs::{
    config::get_config,
    database::{connect_db, create_schemas, populate_tables},
};

#[derive(Clone)]
#[allow(unused)]
pub struct ApplicationState {
    pub timestamp: Instant,
    pub username: String,
    pub weather_location_lat: f64,
    pub weather_location_long: f64,
}

pub fn main() -> ApplicationState {
    let start_time = Instant::now();

    let config = get_config()
        .map_err(|e| {
            println!("Config Error: {}", e);
            process::exit(1);
        })
        .unwrap();

    let conn = connect_db().unwrap();
    create_schemas(&conn).unwrap();
    populate_tables(&conn, &config.clone()).unwrap();

    // Close db connection
    conn.close().unwrap();

    return ApplicationState {
        timestamp: start_time,
        username: config.clone().username,
        weather_location_lat: config.clone().weather_location[0],
        weather_location_long: config.clone().weather_location[1],
    };
}
