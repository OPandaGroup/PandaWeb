use serde::Deserialize;
use sqlx::PgPool;
#[derive(Deserialize)]
pub struct Config {
    pub max_connections: u32,
    pub min_connections: u32,
}
//This is the shared pool
pub struct Cache {
    pub pool: PgPool,
}
impl Cache {
    pub async fn init() -> Self {
        todo!()
    }
}
