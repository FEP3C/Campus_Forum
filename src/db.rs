use sqlx::MySqlPool;

pub async fn create_connection() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", database_url);
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn establish_connection() -> MySqlPool {
    // 调用 create_connection 函数创建 MySqlPool
    let pool = create_connection().await;
    println!("Connection established successfully.");

    // 返回已建立的连接池
    pool
}
