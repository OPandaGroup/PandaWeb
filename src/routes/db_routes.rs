use ntex::http;
use ntex::web;
#[web::get]
async fn get_user() -> http::Response {}
