use ntex::web;
use routes::{
    db_routes::{delete_user, insert_user, select_user, update_user},
    url_routes::{about_css, about_html, about_js, index_css, index_html, index_js, register_html},
};
const SERVER_ADDR: &str = "127.0.0.1:3012";
pub mod config;
pub mod db;
pub mod routes;
use sqlx::PgPool;
pub struct Cache {
    pub pool: PgPool,
}
impl Cache {
    pub async fn init() -> Self {
        Self {
            pool: db::init_pg_pool(),
        }
    }
    pub async fn run(self) {
        web::HttpServer::new(|| {
            web::App::new()
                .state(Cache::init())
                .service(index_html)
                .service(index_css)
                .service(index_js)
                .service(register_html)
                .service(about_html)
                .service(about_css)
                .service(about_js)
                .service(insert_user)
                .service(update_user)
                .service(delete_user)
                .service(select_user)
        })
        .bind(SERVER_ADDR)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
