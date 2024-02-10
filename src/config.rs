use serde::Deserialize;

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
