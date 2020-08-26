use super::users;
use actix_web::web;
use serde::Serialize;

pub fn new(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").route(web::get().to(index)))
        .service(
            web::scope("/api/v1")
                .service(web::resource("/users").route(web::get().to(users::list)))
                .service(web::resource("/users/{id}").route(web::get().to(users::detail))),
        );
}

async fn index() -> web::Json<IndexResponse> {
    web::Json(IndexResponse {
        message: "Hello from API goat!",
    })
}

#[derive(Serialize)]
struct IndexResponse {
    message: &'static str,
}
