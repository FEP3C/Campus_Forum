use jsonwebtoken::{Algorithm, encode, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // subject (通常是用户ID或用户名)
    pub exp: usize,   // 过期时间
    // 你可以添加其他你需要的字段
}

pub fn generate_jwt(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    let key = EncodingKey::from_secret("my_secret_key".as_ref());
    let header = jsonwebtoken::Header::new(Algorithm::HS256); // 确保这里导入的是jsonwebtoken库的Header

    let token = encode(&header, &claims, &key)?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> bool {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret_key".to_string()); // 确保这是您希望使用的默认值

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .is_ok()
}
