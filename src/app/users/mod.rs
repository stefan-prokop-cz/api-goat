use super::database::user::User;
use actix_web::body::Body;
use actix_web::web::Json;
use actix_web::HttpResponse;

pub async fn list() -> Json<Vec<User>> {
    // todo: this is for testing only
    Json(vec![User {
        id: 1,
        name: Some(String::from("Test")),
        surname: Some(String::from("Testovic")),
        username: String::from("test.testovic"),
        password: String::from("myStrongPassword"),
    }])
}
