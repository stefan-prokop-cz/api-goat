use crate::app::config;
use diesel::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub fn connect() -> r2d2::Pool<ConnectionManager<MysqlConnection>> {
    let config = config::Config::get();
    let manager = ConnectionManager::<MysqlConnection>::new(config.database_url);
    let pool = r2d2::Pool::new(manager)
        .unwrap_or_else(|error| panic!("Cannot connect to MySQL: {}", error));
    info!("Successfully connected to MySQL");
    pool
}
