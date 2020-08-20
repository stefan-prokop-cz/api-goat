#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

mod app;

use app::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
