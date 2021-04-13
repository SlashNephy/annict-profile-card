use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default="default_http_addr")]
    pub http_addr: String,
    pub annict_token: String
}

fn default_http_addr() -> String { "0.0.0.0:8080".to_string() }

pub fn load() -> Config {
    envy::from_env().unwrap_or_else(|_| panic!("failed to load env"))
}
