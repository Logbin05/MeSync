use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
  pub token: String
}

impl Config {
    pub fn from_env() -> Self {
      dotenv().ok();
      let token = env::var("TOKEN_BOT").expect("Missing TOKEN_BOT in .env");
      Self { token }
    }
}