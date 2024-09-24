use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::models::{LoginUser, User};
use crate::auth::create_jwt;

pub async fn login_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&form.username)
        .fetch_one(pool.get_ref())
        .await;

    match user {
        Ok(user) => {
            // 验证密码
            if verify(&form.password, &user.password_hash).unwrap() {
                let token = create_jwt(&user.username);
                return HttpResponse::Ok().json(format!("{{\"token\": \"{}\"}}", token));
            }
            HttpResponse::Unauthorized().body("Invalid credentials")
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

pub async fn register_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    // 先检查用户名是否已存在
    let existing_user = sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    if existing_user.is_some() {
        return HttpResponse::Conflict().body("Username already exists.");
    }

    // 对密码进行哈希
    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();

    // 插入新用户到数据库
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
