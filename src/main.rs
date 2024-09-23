mod auth;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to the database.");

    println!("Connected to the database!");
    println!("Server is running at http://localhost:8080");
    println!("Quit the server by pressing Ctrl+C");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/login", web::post().to(handlers::login_user))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
