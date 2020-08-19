use super::config::Config;
use super::router;
use actix_web::{dev, App, HttpServer};

pub fn start() -> dev::Server {
    Config::load();
    let config = Config::get();
    HttpServer::new(|| App::new().configure(router::new))
        .bind(format!("127.0.0.1:{}", config.server_port))
        .unwrap_or_else(|error| {
            panic!("Could not bind server to address 127.0.0.1:3000: {}", error)
        })
        .run()
}
