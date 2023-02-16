use lazy_static::lazy_static;

lazy_static! {
    pub static ref SECRET_KEY: String = dotenv_codegen::dotenv!("SECRET_KEY").to_owned();
    pub static ref REDIS_URL: String = dotenv_codegen::dotenv!("REDIS_URL").to_owned();
    pub static ref SMS_HOST: String = dotenv_codegen::dotenv!("SMS_HOST").to_owned();
}
