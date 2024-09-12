use ntex::{http, web};

static INDEX_HTML: &str = include_str!("../../front/HTML/index.html");
static INDEX_CSS: &str = include_str!("../../front/CSS/index.css");
static INDEX_JS: &str = include_str!("../../front/JS/index.js");
static REGISTER_HTML: &str = include_str!("../../front/HTML/register.html");
static REGISTER_CSS: &str = include_str!("../../front/CSS/register.css");
static REGISTER_JS: &str = include_str!("../../front/JS/register.js");
static ABOUT_HTML: &str = include_str!("../../front/HTML/about.html");
static ABOUT_CSS: &str = include_str!("../../front/CSS/about.css");
static ABOUT_JS: &str = include_str!("../../front/JS/about.js");

#[web::get("/")]
pub async fn index_html() -> http::Response {
    http::Response::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}
#[web::get("index.css")]
pub async fn index_css() -> http::Response {
    http::Response::Ok()
        .content_type("text/css")
        .body(INDEX_CSS)
}
#[web::get("index.js")]
pub async fn index_js() -> http::Response {
    http::Response::Ok()
        .content_type("text/javascript")
        .body(INDEX_JS)
}
#[web::get("/register")]
pub async fn register_html() -> http::Response {
    http::Response::Ok()
        .content_type("text/html")
        .body(REGISTER_HTML)
}
// #[web::get("register.css")]
// pub async fn register_css() -> http::Response {
//     http::Response::Ok()
//         .content_type("text/css")
//         .body(REGISTER_CSS)
// }
// #[web::get("register.js")]
// pub async fn register_js() -> http::Response {
//     http::Response::Ok()
//         .content_type("text/javascript")
//         .body(REGISTER_JS)
// }
#[web::get("/about")]
pub async fn about_html() -> http::Response {
    http::Response::Ok()
        .content_type("text/html")
        .body(ABOUT_HTML)
}
#[web::get("about.css")]
pub async fn about_css() -> http::Response {
    http::Response::Ok()
        .content_type("text/css")
        .body(ABOUT_CSS)
}
#[web::get("about.js")]
pub async fn about_js() -> http::Response {
    http::Response::Ok()
        .content_type("text/javascript")
        .body(ABOUT_JS)
}
