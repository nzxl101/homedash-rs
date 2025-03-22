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
}

pub fn main() -> ApplicationState {
    let start_time = Instant::now();

    let config = get_config().map_err(|e| {
        println!("Config Error: {}", e);
        process::exit(1);
    });

    let conn = connect_db().unwrap();
    create_schemas(&conn).unwrap();
    populate_tables(&conn, &config.unwrap()).unwrap();

    // Close db connection
    conn.close().unwrap();

    return ApplicationState {
        timestamp: start_time,
    };
}
