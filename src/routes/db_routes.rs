use crate::db::User;
use crate::Cache;
use ntex::{http, web};
use uuid::Uuid;
const INSERT_USER: &str = r#"
    INSERT INTO user (id, name, email, password, description, programe_link)
    VALUES ($1, $2, $3, $4, $5, $6)"#;
const UPDATE_USER: &str = r#"
    UPDATE user SET name = $1, email = $2, password = $3, description = $4, programe_link = $5
    WHERE id = $6"#;
const SELECT_USER: &str = r#"
    SELECT * FROM user WHERE id = $1"#;
const DELETE_USER: &str = r#"
    DELETE FROM user WHERE id = $1"#;
#[web::post("/user")]
pub async fn insert_user(
    user: web::types::Json<User>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let user = user.into_inner();
    let pool = &cache.pool;
    let id = Uuid::new_v4().to_string();
    match sqlx::query(INSERT_USER)
        .bind(id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.description)
        .bind(&user.programe_link)
        .execute(pool)
        .await
    {
        Ok(_) => http::Response::Ok().finish(),
        Err(_) => http::Response::BadRequest().finish(),
    }
}
#[web::delete("/user/{id}")]
// I want to get the id from the path
pub async fn delete_user(
    id: web::types::Path<String>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let id = id.into_inner();
    let pool = &cache.pool;
    match sqlx::query(DELETE_USER).bind(id).execute(pool).await {
        Ok(_) => http::Response::Ok().finish(),
        Err(_) => http::Response::BadRequest().finish(),
    }
}

#[web::patch("/user/{id}")]
pub async fn update_user(
    id: web::types::Path<String>,
    user: web::types::Json<User>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let id = id.into_inner();
    let user = user.into_inner();
    let pool = &cache.pool;
    match sqlx::query(UPDATE_USER)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.description)
        .bind(&user.programe_link)
        .bind(id)
        .execute(pool)
        .await
    {
        Ok(_) => http::Response::Ok().finish(),
        Err(_) => http::Response::BadRequest().finish(),
    }
}
#[web::get("/user/{id}")]
// This function will be used for admins.
// I want the response body to become a JSON of the user.
pub async fn select_user(
    id: web::types::Path<String>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let id = id.into_inner();
    match sqlx::query_as::<_, User>(SELECT_USER)
        .bind(id)
        .fetch_one(&cache.pool)
        .await
    {
        Ok(user) => http::Response::Ok().json(&user),
        Err(_) => http::Response::BadRequest().finish(),
    }
}
