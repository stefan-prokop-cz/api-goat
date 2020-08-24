use crate::app::config;
use diesel::pg::PgConnection;
use diesel::Connection;

pub mod schema;
pub mod user;

pub fn connect() -> PgConnection {
    let config = config::Config::get();
    let connection = PgConnection::establish(&config.database_url)
        .unwrap_or_else(|error| panic!("Cannot connect to Postgres: {}", error));
    info!("Successfully connected to Postgres");
    connection
}
