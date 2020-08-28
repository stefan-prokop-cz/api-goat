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

// todo: this is for testing only, remove error
pub async fn detail() -> HttpResponse<Body> {
    HttpResponse::NotFound()
        .content_type("application/json")
        .body(format!(r#"{{ "error": "User not found" }}"#))
}

#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn hash_password() {
        let password = "myPassword";
        let hashed = hash(&password, DEFAULT_COST).expect("Cannot hash password");
        assert_ne!(hashed, password);
        assert_eq!(
            verify(&password, &hashed).expect("Cannot verify password"),
            true
        );
        assert_eq!(
            verify("mypassword", &hashed).expect("Cannot verify password"),
            false
        );
    }
}
