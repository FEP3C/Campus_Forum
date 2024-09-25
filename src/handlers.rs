use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::LoginUser;
use crate::auth::create_jwt;
use serde_json::json;
// 登录和注册接口

pub async fn login_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    match sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(user)) => {
            // 验证密码
            if bcrypt::verify(&form.password, &user.password_hash).unwrap() {
                // 生成 JWT
                let token = create_jwt(&user.username).unwrap(); // 使用一个统一的 JWT 创建函数
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
    match sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().body("Username already exists.");
        }
        Ok(None) => {
            // 哈希密码
            let password_hash = hash(&form.password, DEFAULT_COST).unwrap();

            // 插入新用户，包含 created_at 列
            match sqlx::query!(
                "INSERT INTO users (username, password_hash, created_at) VALUES (?, ?, NOW())",
                &form.username,
                password_hash
            )
            .execute(pool.get_ref())
            .await {
                Ok(_) => HttpResponse::Created().finish(),
                Err(_) => HttpResponse::InternalServerError().body("Failed to create user."),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error querying user."),
    }
}
