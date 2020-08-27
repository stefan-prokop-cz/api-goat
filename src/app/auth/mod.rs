use super::config;
use super::database::user::User;
use actix_web::web::Json;
use hmac::{Hmac, NewMac};
use jwt::{Claims, RegisteredClaims, Token, Header, AlgorithmType, SignWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct SignInResponse {
    user: User,
    credentials: CredentialsResponse,
}

#[derive(Serialize)]
pub struct CredentialsResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    refresh_expires_in: i32,
}

pub async fn sign_in_post() -> Json<SignInResponse> {
    Json(
        sign_user_in(Credentials {
            password: "",
            username: "",
        })
        .unwrap(),
    )
}

pub async fn sign_in_get() -> Json<SignInResponse> {
    Json(
        sign_user_in(Credentials {
            password: "",
            username: "",
        })
        .unwrap(),
    )
}

pub struct Credentials {
    username: &'static str,
    password: &'static str,
}

pub fn sign_user_in(credentials: Credentials) -> Option<SignInResponse> {
    let user = User {
        username: String::from(credentials.username),
        password: String::from(credentials.password),
        name: None,
        surname: None,
        id: 1,
    };
    let credentials = get_credentials(&user);
    Some(SignInResponse { user, credentials })
}

pub fn get_credentials(user: &User) -> CredentialsResponse {
    let mut private_claims = BTreeMap::new();
    private_claims.insert(String::from("username"), json!(user.username.clone()));
    private_claims.insert(String::from("password"), json!(user.password.clone()));
    private_claims.insert(String::from("id"), json!(user.id.to_string()));
    let claims = Claims {
        registered: RegisteredClaims {
            subject: Some(user.id.to_string()),
            expiration: Some(150000000),
            ..Default::default()
        },
        private: private_claims,
    };
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let key: Hmac<Sha384> = Hmac::new_varkey(config::Config::get().token_secret.as_bytes()).unwrap();
    let token = Token::new(header, claims)
        .sign_with_key(&key)
        .unwrap();
    CredentialsResponse {
        access_token: token.as_str().to_string(),
        refresh_token: token.as_str().to_string(),
        expires_in: 3600,
        refresh_expires_in: 150000000,
    }
}
