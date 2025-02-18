use std::env;

use lazy_static::lazy_static;

lazy_static!(
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();
    pub static ref MAIL_PORT: u16 = mail_port();
    pub static ref MAIL_HOST: String = mail_host();
    pub static ref MAIL_USERNAME: String = mail_username();
    pub static ref MAIL_PASSWORD: String = mail_password();
    pub static ref MAIL_FROM_ADDRESS: String = mail_from_address();
    pub static ref AFRICAS_TALKING_API_KEY: String = africas_talking_api_key();
    pub static ref AFRICAS_TALKING_USERNAME: String = africas_talking_username();
    pub static ref EMAIL: String = email();
    pub static ref PASSWORD: String = password();
    pub static ref FRONTEND_URL: String = frontend_url();
    pub static ref SESSION_EXPIRATION_TIME: i64 = session_expiration_time();
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

fn set_max_file_size() -> u64 {
    dotenv::dotenv().ok();
    env::var("MAX_FILE_SIZE")
        .unwrap_or("10485760".to_owned())
        .parse::<u64>()
        .expect("Can't parse that file size")
}

fn mail_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("MAIL_PORT")
        .expect("Environment variable 'MAIL_PORT' is required but not set.")
        .parse::<u16>()
        .expect("Failed to parse 'MAIL_PORT' as a valid u16 value.")
}

fn mail_host() -> String {
    dotenv::dotenv().ok();
    env::var("MAIL_HOST")
        .expect("Environment variable 'MAIL_HOST' is required but not set.")
}

fn mail_username() -> String {
    dotenv::dotenv().ok();
    env::var("MAIL_USERNAME")
        .expect("Environment variable 'MAIL_USERNAME' is required but not set.")
}

fn mail_password() -> String {
    dotenv::dotenv().ok();
    env::var("MAIL_PASSWORD")
        .expect("Environment variable 'MAIL_PASSWORD' is required but not set.")
}

fn mail_from_address() -> String {
    dotenv::dotenv().ok();
    env::var("MAIL_FROM_ADDRESS")
        .expect("Environment variable 'MAIL_FROM_ADDRESS' is required but not set.")
}

fn africas_talking_api_key() -> String {
    dotenv::dotenv().ok();
    env::var("AFRICAS_TALKING_API_KEY")
        .expect("Environment variable 'AFRICAS_TALKING_API_KEY' is required but not set.")
}

fn africas_talking_username() -> String {
    dotenv::dotenv().ok();
    env::var("AFRICAS_TALKING_USERNAME")
        .expect("Environment variable 'AFRICAS_TALKING_USERNAME' is required but not set.")
}

fn email() -> String {
    dotenv::dotenv().ok();
    env::var("EMAIL")
        .expect("Environment variable 'EMAIL' is required but not set.")
}

fn password() -> String {
    dotenv::dotenv().ok();
    env::var("PASSWORD")
        .expect("Environment variable 'PASSWORD' is required but not set.")
}

fn frontend_url() -> String {
    dotenv::dotenv().ok();
    env::var("FRONTEND_URL_ONE")
        .expect("Environment variable 'FRONTEND_URL_ONE' is required but not set.")
}

fn session_expiration_time() -> i64 {
    dotenv::dotenv().ok();
    env::var("SESSION_EXPIRATION_TIME")
    .expect("Environment variable 'SESSION_EXPIRATION_TIME' is required but not set.")
    .parse::<i64>()
    .expect("Failed to parse 'SESSION_EXPIRATION_TIME' as a valid i64 value.")
}