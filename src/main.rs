use actix_web::{get, App, HttpServer, Responder, web::Data};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Campus Forum!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // 创建数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    println!("Connected to the database!");
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone())) // 使用 app_data 代替 data
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
