use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{LoginUser, RegisterUser};
use crate::auth::generate_jwt;
use serde_json::json;

pub async fn login_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<LoginUser>,
) -> HttpResponse {
    match sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(user)) => {
            match bcrypt::verify(&form.password, &user.password_hash) {
                Ok(true) => {
                    let token = generate_jwt(&user.username).unwrap();
                    HttpResponse::Ok().json(json!({ "token": token }))
                },
                Ok(false) => HttpResponse::Unauthorized().body("Invalid password."),
                Err(e) => {
                    eprintln!("Password verification error: {}", e);
                    HttpResponse::InternalServerError().body("Error verifying password.")
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found."),
        Err(e) => {
            eprintln!("Database query error: {}", e);
            HttpResponse::InternalServerError().body("Error querying user.")
        }
    }
}

pub async fn register_user(
    pool: web::Data<MySqlPool>,
    form: web::Json<RegisterUser>,
) -> HttpResponse {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().body("Failed to start transaction.");
        }
    };

    match sqlx::query!("SELECT * FROM users WHERE username = ?", &form.username)
        .fetch_optional(&mut tx)
        .await {
        Ok(Some(_)) => {
            HttpResponse::Conflict().body("Username already exists.")
        }
        Ok(None) => {
            let password_hash = match hash(&form.password, DEFAULT_COST) {
                Ok(hash) => hash,
                Err(e) => {
                    eprintln!("Password hashing error: {}", e);
                    return HttpResponse::InternalServerError().body("Failed to hash password.");
                }
            };

            match sqlx::query!(
                "INSERT INTO users (username, password_hash, created_at, email) VALUES (?, ?, NOW(), ?)",
                &form.username,
                password_hash,
                &form.email
            )
            .execute(&mut tx)
            .await {
                Ok(result) => {
                    if let Err(e) = tx.commit().await {
                        eprintln!("Failed to commit transaction: {}", e);
                        return HttpResponse::InternalServerError().body("Failed to commit transaction.");
                    }
                    println!("User created: {:?}", result);
                    HttpResponse::Created().json(json!({ "message": "User created successfully" }))
                }
                Err(e) => {
                    eprintln!("Failed to create user: {}", e);
                    HttpResponse::InternalServerError().body("Failed to create user.")
                }
            }
        }
        Err(e) => {
            eprintln!("Error querying user: {}", e);
            HttpResponse::InternalServerError().body("Error querying user.")
        }
    }
}