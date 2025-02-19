use actix_web::web::{self};

use crate::controllers;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api")
            .service(controllers::listings::create_listing)
            .service(controllers::listings::update_listing)
            .service(controllers::listings::get_all_listings)
            .service(controllers::listings::delete_listing)
        );
}