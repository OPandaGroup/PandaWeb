const INSERT_USER: &str = r#"
    INSERT INTO user (name, email, password, description, programe_link)
    VALUES ($1, $2, $3, $4, $5)"#;
const UPDATE_USER: &str = r#"
    UPDATE user SET name = $1, email = $2, password = $3, description = $4, programe_link = $5
    WHERE id = $6"#;
const SELECT_USER: &str = r#"
    SELECT * FROM user WHERE id = $1"#;
const DELETE_USER: &str = r#"
    DELETE FROM user WHERE id = $1"#;

static PRI_KEY: LazyLock<RsaPrivateKey> = LazyLock::new(|| {
    RsaPrivateKey::new(&mut rand::thread_rng(), 2048).expect("failed to generate a key")
});
static PUB_KEY: LazyLock<RsaPublicKey> = LazyLock::new(|| PRI_KEY.to_public_key());

use crate::db::User;
use crate::Cache;
use ntex::{http, web};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::sync::LazyLock;
#[web::post("/user")]
pub async fn insert_user(
    user: web::types::Json<User>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let password = unsafe {
        String::from_utf8_unchecked(
            PUB_KEY
                .encrypt(
                    &mut rand::thread_rng(),
                    Pkcs1v15Encrypt,
                    user.password.as_bytes(),
                )
                .expect("failed to encrypt"),
        )
    };
    let pool = &cache.pool;
    match sqlx::query(INSERT_USER)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&password)
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
    id: web::types::Path<i64>,
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
    id: web::types::Path<i64>,
    user: web::types::Json<User>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let id = id.into_inner();
    let user = user.into_inner();
    let pool = &cache.pool;
    match sqlx::query(UPDATE_USER)
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
#[web::get("/user/{id}")]
pub async fn select_user(
    id: web::types::Path<i64>,
    cache: web::types::State<Cache>,
) -> http::Response {
    let id = id.into_inner();
    let pool = &cache.pool;
    match sqlx::query_as::<_, User>(SELECT_USER)
        .bind(id)
        .fetch_one(pool)
        .await
    {
        Ok(mut user) => {
            user.password = unsafe {
                String::from_utf8_unchecked(
                    PRI_KEY
                        .decrypt(Pkcs1v15Encrypt, user.password.as_bytes())
                        .expect("failed to decrypt"),
                )
            };
            http::Response::Ok().json(&user)
        }
        Err(_) => http::Response::BadRequest().finish(),
    }
}
