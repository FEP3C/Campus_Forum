use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub email: String
}

#[allow(dead_code)]
struct User {
    id: i32,
    username: String,
    password_hash: String,
}
