use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref APP_NAME: String = env_or_default("APP_NAME");
    pub static ref APP_SECRET: String = env_or_default("APP_SECRET");
    pub static ref DB_URL: String = env_or_default("DB_URL");
    pub static ref DB_USERNAME: String = env_or_default("DB_USERNAME");
    pub static ref DB_PASSWORD: String = env_or_default("DB_PASSWORD");
    pub static ref DB_AUTH: String = general_purpose::STANDARD
        .encode(format!("{}:{}", DB_USERNAME.as_str(), DB_PASSWORD.as_str()).as_bytes());
    pub static ref REDIS_URL: String = env_or_default("REDIS_URL");
    pub static ref SMS_HOST: String = env_or_default("SMS_HOST");
    pub static ref SMTP_HOST: String = env_or_default("SMTP_HOST");
    pub static ref SMTP_PORT: String = env_or_default("SMTP_PORT");
    pub static ref SMTP_USERNAME: String = env_or_default("SMTP_USERNAME");
    pub static ref SMTP_PASSWORD: String = env_or_default("SMTP_PASSWORD");
    pub static ref SMTP_FROM: String = env_or_default("SMTP_FROM");
}

fn env_or_default(key: &str) -> String {
    match dotenvy::var(key) {
        Ok(val) => val,
        Err(_) => match env::var(key) {
            Ok(val) => val,
            Err(_) => panic!("{key} environment variable not found"),
        },
    }
}
