use ntex::web;
use routes::about_css;
use routes::about_html;
use routes::index_css;
use routes::index_html;
use routes::index_js;
use routes::register_css;
use routes::register_html;
use routes::register_js;
use state::Cache;
const SERVER_ADDR: &str = "127.0.0.1:3012";
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
                .service(about_html)
        })
        .bind(SERVER_ADDR)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}