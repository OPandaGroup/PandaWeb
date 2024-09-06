use crate::state::Cache;
use ntex::web;
use routes::{
    url_routes::about_css, url_routes::about_html, url_routes::about_js, url_routes::index_css,
    url_routes::index_html, url_routes::index_js, url_routes::register_css,
    url_routes::register_html, url_routes::register_js,
};

const SERVER_ADDR: &str = "127.0.0.1:3012";
pub mod db;
pub mod routes;
pub mod state;
pub struct Application {
    cache: Cache,
}
impl Application {
    async fn init() -> Self {
        let cache = Cache::init().await;
        Self { cache }
    }
}
impl Application {
    pub async fn run(&mut self) {
        web::HttpServer::new(|| {
            web::App::new()
                .service(index_html)
                .service(index_css)
                .service(index_js)
                .service(register_html)
                .service(register_css)
                .service(register_js)
                .service(about_html)
                .service(about_css)
                .service(about_js)
        })
        .bind(SERVER_ADDR)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
