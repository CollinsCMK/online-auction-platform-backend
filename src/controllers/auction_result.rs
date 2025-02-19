use actix_web::{get, web};
use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, Condition, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[derive(Debug, FromQueryResult)]
struct BidGetResult {
    name: String,
    listing_title: String,
    amount: Decimal,
    created_at: NaiveDateTime,
}

pub async fn create_auction_result(
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let now = Utc::now().naive_utc();

    let active_listing_ids = entity::auctions::Entity::find()
        .inner_join(entity::listings::Entity)
        .filter(
            Condition::all()
                .add(entity::auctions::Column::StartTime.lte(now))
                .add(entity::auctions::Column::EndTime.gte(now))
        )
        .select_only()
        .column(entity::listings::Column::Id)
        .into_tuple::<i32>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?;

    let bids = entity::bids::Entity::find()
        .filter(entity::bids::Column::DeletedAt.is_null())
        .filter(entity::bids::Column::ListingId.is_in(active_listing_ids))
        .inner_join(entity::listings::Entity)
        .inner_join(entity::users::Entity)
        .select_only()
        .column(entity::users::Column::Name)
        .column_as(entity::listings::Column::Title, "listing_title")
        .column(entity::bids::Column::Amount)
        .column(entity::bids::Column::CreatedAt)
        .into_model::<BidGetResult>()
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
                "name": row.name,
                "listing_title": row.listing_title,
                "amount": row.amount,
                "created_at": row.created_at,
            })
        })
        .collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(
        json!({
            "message": "Auction result created successfully".to_string()
        })
    )))
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