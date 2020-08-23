use super::config::Config;
use super::database;
use super::router;
use actix_web::middleware::Logger;
use actix_web::{dev, App, HttpServer};
use env_logger;

pub fn start() -> dev::Server {
    Config::load();
    let config = Config::get();
    env_logger::init();
    info!("Running with {:?}", config);
    database::connect();
    HttpServer::new(|| App::new().configure(router::new).wrap(Logger::default()))
        .bind(format!("0.0.0.0:{}", config.server_port))
        .unwrap_or_else(|error| {
            panic!(
                "Could not bind server to address 127.0.0.1:{}: {}",
                config.server_port, error
            )
        })
        .run()
}
