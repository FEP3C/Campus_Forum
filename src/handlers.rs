use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use bcrypt::verify;
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
