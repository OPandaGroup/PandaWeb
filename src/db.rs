use crate::state::Config;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use tokio::fs;
const PG_URL: &str = "postgres://lym:1234@localhost/panda";
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    // Emails and passwords will be encrypted using prime numbers
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub description: String,
    pub programe_link: String,
}
pub async fn init_pg_pool() -> Arc<PgPool> {
    Arc::new(match fs::read_to_string("config.toml").await {
        Ok(config) => match toml::from_str::<Config>(&config) {
            Ok(config) => PgPoolOptions::new()
                .max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect(PG_URL)
                .await
                .unwrap(),
            Err(_) => PgPoolOptions::new().connect(PG_URL).await.unwrap(),
        },
        Err(_) => PgPoolOptions::new().connect(PG_URL).await.unwrap(),
    })
}
