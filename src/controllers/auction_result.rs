use std::{collections::HashMap, time::Duration};

use actix_web::{get, rt::time::interval, web};
use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, Set, ActiveModelTrait};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response, whatsapp::send_whatsapp_message};

#[derive(Debug, FromQueryResult)]
struct BidGetResult {
    listing_id: i32,
    winning_bid_id: i32,
    winning_user_id: i32,
    amount: Decimal,
}

pub async fn create_auction_result(db: DatabaseConnection) -> Result<(), ApiResponse> {
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        
        if let Err(err) = process_auction_results(&db).await {
            eprintln!("Error processing auction results: {:?}", err);
        }
    }
}

async fn process_auction_results(db: &DatabaseConnection) -> Result<(), ApiResponse> {
    let now = Utc::now().naive_utc();

    // Get ended auctions
    let auctions_ended = entity::auctions::Entity::find()
        .filter(entity::auctions::Column::DeletedAt.is_null())
        .filter(entity::auctions::Column::EndTime.lt(now))
        .all(db)
        .await
        .map_err(|err| ApiResponse::new(500, response(json!({ "error": err.to_string() }))))?;

    let auction_ids: Vec<i32> = auctions_ended.iter().map(|auction| auction.id).collect();

    if auction_ids.is_empty() {
        return Ok(()); // No ended auctions, skip processing
    }

    // Get highest bids
    let highest_bids = entity::bids::Entity::find()
        .inner_join(entity::listings::Entity)
        .inner_join(entity::users::Entity)
        .filter(entity::bids::Column::DeletedAt.is_null())
        .filter(entity::listings::Column::AuctionId.is_in(auction_ids.clone()))
        .select_only()
        .column_as(entity::bids::Column::Id, "winning_bid_id")
        .column_as(entity::users::Column::Id, "winning_user_id")
        .column_as(entity::listings::Column::Id, "listing_id")
        .column(entity::bids::Column::Amount)
        .into_model::<BidGetResult>()
        .all(db)
        .await
        .map_err(|err| ApiResponse::new(500, response(json!({ "error": err.to_string() }))))?;

    let mut highest_bids_map: HashMap<i32, BidGetResult> = HashMap::new();
    for bid in highest_bids {
        match highest_bids_map.get(&bid.listing_id) {
            Some(current_highest) => {
                if bid.amount > current_highest.amount {
                    highest_bids_map.insert(bid.listing_id, bid);
                }
            }
            None => {
                highest_bids_map.insert(bid.listing_id, bid);
            }
        }
    }

    let all_listings = entity::listings::Entity::find()
        .filter(entity::listings::Column::AuctionId.is_in(auction_ids))
        .all(db)
        .await
        .map_err(|err| ApiResponse::new(500, response(json!({ "error": err.to_string() }))))?;

    for listing in &all_listings {
        let title = listing.title.clone();

        if let Some(bid) = highest_bids_map.get(&listing.id) {
            let user = entity::users::Entity::find_by_id(bid.winning_user_id)
                .one(db)
                .await
                .map_err(|err| ApiResponse::new(500, response(json!({ "error": err.to_string() }))))?
                .map(|u| u.name)
                .unwrap_or_else(|| "Unknown User".to_string());

            let _auction_result = entity::auction_results::ActiveModel {
                listing_id: Set(listing.id),
                winning_bid_id: Set(bid.winning_bid_id),
                winning_user_id: Set(bid.winning_user_id),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(|err| ApiResponse::new(500, response(json!({ "error": err.to_string() }))))?;

            let message = format!(
                "Auction Result:\nAuction ID: {}\nListing: {}\nWinning Bid: {}\nWinner: {}",
                listing.auction_id, title, bid.amount, user
            );

            send_whatsapp_message("254792315642", &message).await?;
        } else {
            let message = format!(
                "Auction Result:\nAuction ID: {}\nListing: {}\nNo bids were placed.",
                listing.auction_id, title
            );

            send_whatsapp_message("254792315642", &message).await?;
        }
    }

    Ok(())
}

#[derive(Debug, FromQueryResult)]
struct AuctionResultDataResult {
    id: i32,
    name: String,
    amount: Decimal,
    title: String,
    created_at: NaiveDateTime,
}

#[get("/auction_results/get")]
pub async fn get_auction_results(
    app_state: web::Data<AppState>
) -> Result<ApiResponse, ApiResponse> {
    let auction_results = entity::auction_results::Entity::find()
        .filter(entity::auction_results::Column::DeletedAt.is_null())
        .inner_join(entity::listings::Entity)
        .inner_join(entity::users::Entity)
        .inner_join(entity::bids::Entity)
        .select_only()
        .column(entity::auction_results::Column::Id)
        .column(entity::users::Column::Name)
        .column(entity::bids::Column::Amount)
        .column(entity::listings::Column::Title)
        .column(entity::auction_results::Column::CreatedAt)
        .into_model::<AuctionResultDataResult>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?
        .into_iter()
        .map(|row| {
            json!({
                "id": row.id,
                "name": row.name,
                "amount": row.amount,
                "title": row.title,
                "created_at": row.created_at,
            })
        })
        .collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(
        json!({
            "auction_results": auction_results,
            "message": "Auction results fetched successfully".to_string()
        })
    )))
}