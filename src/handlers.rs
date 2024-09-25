use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::models::{LoginUser, User};
use crate::auth::create_jwt;
use jsonwebtoken::{encode, Header};

pub async fn login_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    let user = sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            // 验证密码
            if bcrypt::verify(&form.password, &user.password_hash).unwrap() {
                // 生成 JWT
                let token = encode(&Header::default(), &user.username, "your_secret_key".as_ref()).unwrap();
                return HttpResponse::Ok().json(json!({ "token": token }));
            }
            HttpResponse::Unauthorized().body("Invalid password.")
        }
        Ok(None) => HttpResponse::NotFound().body("User not found."),
        Err(_) => HttpResponse::InternalServerError().body("Error querying user."),
    }
}

pub async fn register_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    // 检查用户名是否已存在
    let existing_user = sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap();

    if existing_user.is_some() {
        return HttpResponse::Conflict().body("Username already exists.");
    }

    // 哈希密码
    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();

    // 插入新用户
    let result = sqlx::query!("INSERT INTO users (username, password_hash) VALUES (?, ?)",
                              &form.username,
                              password_hash)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user."),
    }
}
