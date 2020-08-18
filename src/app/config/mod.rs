use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_port: u16,
}

impl Config {
    pub fn get() -> Config {
        Config {
            server_port: env::var("SERVER_PORT")
                .unwrap_or(String::from("3000"))
                .parse::<u16>()
                .unwrap(),
        }
    }
    pub fn load() {
        dotenv().unwrap_or_else(|error| panic!("Cannot load config: {}", error));
    }
}
