mod users;
use actix_web::{App, dev, get, HttpServer, web};

pub fn start() -> dev::Server {
    HttpServer::new(|| App::new()
        .configure(get_routes))
        .bind("127.0.0.1:3000")
        .unwrap_or_else(|error| panic!("Could not bind server to address 127.0.0.1:3000: {}", error))
        .run()
}

fn get_routes(app: &mut web::ServiceConfig) {
    app
        .service(index)
        .service(
            web::scope("/api/v1")
                .service(
                    web::resource("/users")
                        .route(web::get().to(users::list))
                )
        );
}

#[get("/")]
async fn index() -> &'static str {
    "Hello API goat!"
}
