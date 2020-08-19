use super::users;
use actix_web::web;

pub fn new(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").route(web::get().to(index)))
        .service(
            web::scope("/api/v1")
                .service(web::resource("/users").route(web::get().to(users::list))),
        );
}

async fn index() -> &'static str {
    "Hello from API goat!"
}
