use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::mysql::MySqlPool;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

// 用户注册请求的数据结构
#[derive(Deserialize)]  // 启用 serde 的反序列化功能
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

// 注册用户的处理函数
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

// `main` 函数是程序的入口点
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // 加载 .env 文件中的环境变量

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = MySqlPool::connect(&database_url).await
        .expect("Failed to connect to the database.");

    println!("Connected to the database!");
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // 使用 app_data 共享数据库连接池
            .route("/register", web::post().to(register_user))  // 注册路由
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
