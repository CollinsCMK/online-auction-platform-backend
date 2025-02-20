use actix_web::web::{self};

use crate::controllers;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api")

            // Start: API's for listings
            .service(controllers::listings::create_listing)
            .service(controllers::listings::update_listing)
            .service(controllers::listings::get_all_listings)
            .service(controllers::listings::delete_listing)
            // End: API's for listings

            // Start: API's for users
            .service(controllers::user::create_user)
            .service(controllers::user::get_all_users)
            .service(controllers::user::get_user)
            .service(controllers::user::delete_user)
            // End: API's for users

            // Start: API's for auctions
            .service(controllers::auction::create_auction)
            .service(controllers::auction::update_auction)
            .service(controllers::auction::get_all_auctions)
            .service(controllers::auction::get_user_auctions)
            .service(controllers::auction::delete_auction)
            // End: API's for auctions

            // Start: API's for auction results
            .service(controllers::auction_result::get_auction_results)
            // End: API's for auction results

            // Start: API's for bids
            .service(controllers::bids::create_bid)
            .service(controllers::bids::get_all_bids)
            .service(controllers::bids::get_active_bids)
            .service(controllers::bids::get_all_user_bids)
            // End: API's for bids
        );
}