use std::fs;

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Endpoint {
    pub address: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct World {
    pub x: u32,
    pub y: u32,
    pub frequency: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub endpoint: Endpoint,
    pub world: World,
}

pub fn open_config() -> Config {
    let config_file = fs::read_to_string("config/config.toml").unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();
    config
}
