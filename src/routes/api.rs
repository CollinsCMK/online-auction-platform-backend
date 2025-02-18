use actix_web::web::{self};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api")
        );
}