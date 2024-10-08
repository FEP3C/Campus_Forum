mod auth;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use dotenv::dotenv;
use serde::Serialize;
use sqlx::mysql::MySqlPool;
use std::env;

// Struct for API response
#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

// Handler for "/api/data"
async fn get_data() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        message: String::from("Hello from the backend!"),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database.");

    println!("Connected to the database!");
    println!("Server is running at http://localhost:8080");
    println!("Quit the server by pressing Ctrl+C");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/login", web::post().to(handlers::login_user))
            .route("/register", web::post().to(handlers::register_user))
            .route("/api/data", web::get().to(get_data)) // Adding the route for /api/data
            .service(Files::new("/", "./static").index_file("index.html")) // Static file service
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
