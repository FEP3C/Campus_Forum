use actix_web::web::Header;
use jsonwebtoken::{encode,decode,Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // subject (通常是用户ID或用户名)
    pub exp: usize,   // 过期时间
    // 你可以添加其他你需要的字段
}

pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    let key = EncodingKey::from_secret("my_secret_key".as_ref());
    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &key)?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> bool {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .is_ok()
}
