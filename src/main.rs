mod app;

use app::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
