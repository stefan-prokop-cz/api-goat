use crate::app::config;
use diesel::mysql::MysqlConnection;
use diesel::Connection;

pub mod schema;
pub mod user;

pub fn connect() -> MysqlConnection {
    let config = config::Config::get();
    let connection = MysqlConnection::establish(&config.database_url)
        .unwrap_or_else(|error| panic!("Cannot connect to MySQL: {}", error));
    info!("Successfully connected to MySQL");
    connection
}
