use std::env;

use lazy_static::lazy_static;

lazy_static!(
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
    pub static ref FRONTEND_URL: String = frontend_url();
    pub static ref SESSION_EXPIRATION_TIME: i64 = session_expiration_time();
    pub static ref WHATSAPP_ACCESS_TOKEN: String = whatsapp_access_token();
    pub static ref WHATSAPP_PHONE_NUMBER_ID: String = whatsapp_phone_number_id();
    pub static ref WHATSAPP_BUSINESS_ACCOUNT_ID: String = whatsapp_business_account_id();
    pub static ref WHATSAPP_PHONE_NUMBER: String = whatsapp_phone_number();
);

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS")
        .expect("Environment variable 'ADDRESS' is required but not set.")
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .expect("Environment variable 'PORT' is required but not set.")
        .parse::<u16>()
        .expect("Failed to parse 'PORT' as a valid u16 value.")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .expect("Environment variable 'DATABASE_URL' is required but not set.")
}

fn set_secret() -> String {
    dotenv::dotenv().ok();
    env::var("SECRET")
        .expect("Environment variable 'SECRET' is required but not set.")
}

fn frontend_url() -> String {
    dotenv::dotenv().ok();
    env::var("FRONTEND_URL")
        .expect("Environment variable 'FRONTEND_URL' is required but not set.")
}

fn session_expiration_time() -> i64 {
    dotenv::dotenv().ok();
    env::var("SESSION_EXPIRATION_TIME")
    .expect("Environment variable 'SESSION_EXPIRATION_TIME' is required but not set.")
    .parse::<i64>()
    .expect("Failed to parse 'SESSION_EXPIRATION_TIME' as a valid i64 value.")
}

fn whatsapp_access_token() -> String {
    dotenv::dotenv().ok();
    env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Environment variable 'WHATSAPP_ACCESS_TOKEN' is required but not set.")
}

fn whatsapp_phone_number_id() -> String {
    dotenv::dotenv().ok();
    env::var("WHATSAPP_PHONE_NUMBER_ID")
        .expect("Environment variable 'WHATSAPP_PHONE_NUMBER_ID' is required but not set.")
}

fn whatsapp_business_account_id() -> String {
    dotenv::dotenv().ok();
    env::var("WHATSAPP_BUSINESS_ACCOUNT_ID")
        .expect("Environment variable 'WHATSAPP_BUSINESS_ACCOUNT_ID' is required but not set.")
}

fn whatsapp_phone_number() -> String {
    dotenv::dotenv().ok();
    env::var("WHATSAPP_PHONE_NUMBER")
        .expect("Environment variable 'WHATSAPP_PHONE_NUMBER' is required but not set.")
}