use actix_web::{get, HttpResponse, web, Error};
use sqlx::MySqlPool;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
    email: Option<String>,
    role: String,
}

#[get("/admin/users")]
async fn get_users(pool: web::Data<MySqlPool>) -> Result<HttpResponse, Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at FROM users"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();
    
}