use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub token_secret: String,
}

impl Config {
    pub fn get() -> Config {
        Config {
            server_port: env::var("SERVER_PORT")
                .unwrap_or(String::from("3000"))
                .parse::<u16>()
                .unwrap(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            token_secret: env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set"),
        }
    }
    pub fn load() {
        dotenv().unwrap_or_else(|error| panic!("Cannot load config: {}", error));
    }
}
