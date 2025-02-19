use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[derive(Debug, Serialize, Deserialize)]
struct BidData {
    listing_id: i32,
    user_id: i32,
    amount: Decimal,
}

impl BidData {
    pub fn validate(&self) -> Result<(), String> {
        if self.listing_id <= 0 {
            return Err("Invalid Listing ID".to_string());
        }

        if self.user_id <= 0 {
            return Err("Invalid User ID".to_string());
        }

        if self.amount <= Decimal::ZERO {
            return Err("Bid amount must be greater than zero".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, FromQueryResult)]
struct BidResult {
    end_time: NaiveDateTime
}

#[post("/bid/create")]
pub async fn create_bid(
    bid_data: web::Json<BidData>,
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    if let Err(err) = bid_data.validate() {
        return Err(ApiResponse::new(500, response(
            json!({
                "error": err.to_string()
            })
        )));
    }

    let listing_model = entity::listings::Entity::find_by_id(bid_data.listing_id)
        .inner_join(entity::auctions::Entity)
        .filter(entity::listings::Column::DeletedAt.is_null())
        .select_only()
        .column(entity::auctions::Column::EndTime)
        .into_model::<BidResult>()
        .one(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?
        .ok_or_else(|| {
            ApiResponse::new(404, response(
                json!({
                    "error": "Listing not found".to_string()
                })
            ))
        })?;

    if listing_model.end_time < Utc::now().naive_utc() {
        return Err(ApiResponse::new(400, response(
            json!({
                "error": "Bidding is closed. The auction has already ended.".to_string()
            })
        )))
    }

    entity::bids::ActiveModel {
        listing_id: Set(bid_data.listing_id.clone()),
        user_id: Set(bid_data.user_id.clone()),
        amount: Set(bid_data.amount.clone()),
        ..Default::default()
    }
        .insert(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?;

    Ok(ApiResponse::new(200, response(
        json!({
            "message": "Bid created successfully".to_string()
        })
    )))
}

#[derive(Debug, FromQueryResult)]
struct BidGetResult {
    name: String,
    listing_title: String,
    amount: Decimal,
    created_at: NaiveDateTime,
}

#[get("/bids/get")]
pub async fn get_all_bids(
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let bids = entity::bids::Entity::find()
        .filter(entity::bids::Column::DeletedAt.is_null())
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
            "bids": bids,
            "message": "Bids fetched successfully".to_string()
        })
    )))
}

#[get("/bids/active")]
pub async fn get_active_bids(
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
            "bids": bids,
            "message": "Bids for active auctions fetched successfully".to_string()
        })
    )))
}