use actix_files as fs;  // 引入 actix-files 库
use actix_web::{web, App, HttpServer, HttpResponse, Responder}; // 确保正确导入 HttpServer
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::mysql::MySqlPool;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

async fn register_user(
    pool: web::Data<MySqlPool>, 
    user: web::Json<RegisterUser>
) -> impl Responder {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(pwd) => pwd,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES (?, ?, ?)
        "#,
        user.username, user.email, hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User registered successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Error registering user."),
    }
}

async fn login_user(
    pool: web::Data<MySqlPool>, 
    user: web::Json<LoginUser>
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        SELECT password_hash FROM users WHERE username = ?
        "#,
        user.username
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let is_valid = verify(&user.password, &record.password_hash).unwrap_or(false);
            if is_valid {
                HttpResponse::Ok().body("Login successful!")
            } else {
                HttpResponse::Unauthorized().body("Invalid password.")
            }
        },
        Err(_) => HttpResponse::Unauthorized().body("User not found."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = MySqlPool::connect(&database_url).await
        .expect("Failed to connect to the database.");

    println!("Connected to the database!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            // 提供静态文件服务
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?  // 确保 bind 正确
    .run()
    .await
}
