use actix_web::{web, HttpResponse, Error, post};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use bcrypt::verify;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct Claims {
    sub: String,
    role: String,
    exp: usize, // 过期时间
}

#[post("/login")]
async fn login(data: web::Json<LoginData>, pool: web::Data<MySqlPool>) -> Result<HttpResponse, Error> {
    let user = sqlx::query!(
        "SELECT username, password, role FROM users WHERE username = ?",
        data.username
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user_row) => {
            if verify(&data.password, &user_row.password).unwrap() {
                let my_claims = Claims {
                    sub: data.username.clone(),
                    role: user_row.role,
                    exp: 2000000000,
                };
                let token = encode(
                    &Header::new(Algorithm::HS256),
                    &my_claims,
                    &EncodingKey::from_secret("your_secret_key".as_ref())
                ).unwrap();

                return Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token, "username": data.username })));
            }
        }
        Err(_) => return Ok(HttpResponse::Unauthorized().body("Invalid credentials")),
    }

    Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
}