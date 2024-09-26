use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod auth;
mod db;
mod admin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(auth::login)
            .service(admin::get_users)
    
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}