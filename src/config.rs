use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub max_connections: u32,
    pub min_connections: u32,
}
