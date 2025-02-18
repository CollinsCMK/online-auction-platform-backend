use std::{error::Error, fmt::Display};
use actix_cors::Cors;
use actix_session::{config::PersistentSession, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{cookie::{Key, SameSite}, middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod utils;
mod routes;
mod controllers;
mod helpers;

#[derive(Debug)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), MainError>{
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (utils::constants::PORT).clone();
    let address = (utils::constants::ADDRESS).clone();
    let database_url = (utils::constants::DATABASE_URL).clone();
    let fronted_url = (utils::constants::FRONTEND_URL).clone();
    let session_expiration_time = (utils::constants::SESSION_EXPIRATION_TIME).clone();
    
    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .map_err(|err| MainError { message: err.to_string() })?;

    HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new( AppState { db: db.clone() } ))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                // .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(actix_web::cookie::time::Duration::minutes(
                            session_expiration_time
                        )
                    ),
                )
                .build(),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    // .allowed_origin(&fronted_url)
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .configure(routes::api::config)
    })
        .bind((address, port))
        .map_err(|err| MainError { message: err.to_string() })?
        .run()
        .await
        .map_err(|err| MainError { message: err.to_string() })
}