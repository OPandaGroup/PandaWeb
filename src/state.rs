use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, PgPool};
const PG_URL: &str = "postgres://lym:1234@localhost/panda";
pub struct Config {
    max_connections: u8,
    min_connections: u8,
}
//This is the shared pool
pub struct Cache {
    pub pool: Arc<PgPool>,
}
impl Cache {
    pub async fn init() -> Self {
        let pool = Arc::new(
            PgPoolOptions::new()
                .max_connections(16)
                .min_connections(4)
                .connect(PG_URL)
                .await
                .unwrap(),
        );

        Self { pool }
    }
}
