use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::mysql::MySqlPool;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

// 用户注册请求的数据结构
#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

// 用户登录请求的数据结构
#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

// 注册用户的处理函数
async fn register_user(
    pool: web::Data<MySqlPool>, 
    user: web::Json<RegisterUser>
) -> impl Responder {
    // 使用 bcrypt 哈希用户密码
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(pwd) => pwd,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // 插入用户数据到数据库
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

// 用户登录的处理函数
async fn login_user(
    pool: web::Data<MySqlPool>, 
    user: web::Json<LoginUser>
) -> impl Responder {
    // 从数据库中查找用户
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
            // 使用 bcrypt 验证密码
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

// `main` 函数是程序的入口点
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // 加载 .env 文件中的环境变量

    // 获取数据库 URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // 连接到数据库
    let pool = MySqlPool::connect(&database_url).await
        .expect("Failed to connect to the database.");

    println!("Connected to the database!");
    println!("Server is running at http://localhost:8080");
    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // 使用 app_data 共享数据库连接池
            .route("/register", web::post().to(register_user))  // 注册路由
            .route("/login", web::post().to(login_user))        // 登录路由
    })
    .bind("127.0.0.1:8080")?  // 监听端口
    .run()
    .await
}
