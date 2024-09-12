use crate::config::Config;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::fs;
const PG_URL: &str = "postgres://lym:1234@localhost/panda";
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    // Emails and passwords will be encrypted using prime numbers
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub description: String,
    pub programe_link: String,
}
pub fn init_pg_pool() -> PgPool {
    match fs::read_to_string("config.toml") {
        Ok(config) => match toml::from_str::<Config>(&config) {
            Ok(config) => PgPoolOptions::new()
                .max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect_lazy(PG_URL)
                .unwrap(),
            Err(_) => PgPoolOptions::new().connect_lazy(PG_URL).unwrap(),
        },
        Err(_) => PgPoolOptions::new().connect_lazy(PG_URL).unwrap(),
    }
}
