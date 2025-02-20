use std::collections::HashMap;

use actix_web::{delete, get, post, put, web};
use chrono::{NaiveDateTime, Utc};
use migration::Expr;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[derive(Debug, Serialize, Deserialize)]
struct AuctionData {
    name: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
} 

impl AuctionData {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }
        
        if self.start_time.to_string().is_empty() {
            return Err("Start time is required".to_string());
        }

        if self.end_time.to_string().is_empty() {
            return Err("End time is required".to_string());
        }
        
        if self.start_time > self.end_time {
            return Err("Start time must be before end time".to_string());
        }

        if self.end_time <= self.start_time {
            return Err("End time must be after start time".to_string());
        }

        Ok(())
    }
}

#[post("/auction/create")]
pub async fn create_auction(
    app_state: web::Data<AppState>,
    auction_data: web::Json<AuctionData>
) -> Result<ApiResponse, ApiResponse> {
    if let Err(err) = auction_data.validate() {
        return Err(ApiResponse::new(500, response(
            json!({
                "error": err.to_string()
            })
        )));
    }

    let _auction_model = entity::auctions::ActiveModel {
        name: Set(auction_data.name.clone()),
        start_time: Set(auction_data.start_time),
        end_time: Set(auction_data.end_time),
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
            "message": "Auction created successfully".to_string()
        })
    )))
}

#[put("/auction/update/{id}")]
pub async fn update_auction(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    auction_data: web::Json<AuctionData>,
) -> Result<ApiResponse, ApiResponse> {
    let auction_id = path.into_inner();

    let auction_model = entity::auctions::Entity::find_by_id(auction_id)
        .filter(entity::auctions::Column::DeletedAt.is_null())
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
                    "error": "Auction data not found".to_string()
                })
            ))
        })?;

    let mut update_auction_model: entity::auctions::ActiveModel = auction_model.to_owned().into();
    update_auction_model.name = Set(auction_data.name.clone());
    update_auction_model.start_time = Set(auction_data.start_time.clone());
    update_auction_model.end_time = Set(auction_data.end_time.clone());
    update_auction_model.updated_at = Set(Utc::now().naive_local());
    update_auction_model
        .update(&app_state.db)
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
            "message": "Auction data updated successfully".to_string()
        })
    )))
}

#[get("/auctions/get")]
pub async fn get_all_auctions(
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let auction_model = entity::auctions::Entity::find()
        .filter(entity::auctions::Column::DeletedAt.is_null())
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?
        .iter()
        .map(|row| {
            json!({
                "id": row.id,
                "name": row.name,
                "start_time": row.start_time,
                "end_time": row.end_time,
                "updated_at": row.updated_at,
            })
        })
        .collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(
        json!({
            "auctions": auction_model,
            "message": "Auction data fetched successfully".to_string()
        })
    )))
}

#[delete("/auction/delete/{id}")]
pub async fn delete_auction(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let aunction_id = path.into_inner();

    let _auction_model = entity::auctions::Entity::delete_by_id(aunction_id)
        .exec(&app_state.db)
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
            "message": "Auction deleted successfully".to_string()
        })
    )))
}

#[get("/auctions/user/{id}")]
pub async fn get_user_auctions(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let user_id = path.into_inner();
    let now = Utc::now().naive_local();

    // Fetch only auctions that are either Not Started or Active (exclude Ended ones)
    let active_auctions = entity::auctions::Entity::find()
        .inner_join(entity::listings::Entity)
        .filter(entity::auctions::Column::EndTime.gte(now))
        .select_only()
        .column(entity::listings::Column::Id)
        .column(entity::auctions::Column::StartTime)
        .column(entity::auctions::Column::EndTime)
        .column(entity::listings::Column::Title)
        .column(entity::listings::Column::Description)
        .column(entity::listings::Column::BasePrice)
        .column(entity::listings::Column::AvailableVolume)
        .into_tuple::<(i32, NaiveDateTime, NaiveDateTime, String, String, Decimal, i32)>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(json!({ "error": err.to_string() })))
        })?;

    let highest_user_bids = entity::bids::Entity::find()
        .filter(entity::bids::Column::UserId.eq(user_id))
        .group_by(entity::bids::Column::ListingId)
        .select_only()
        .column(entity::bids::Column::ListingId)
        .column_as(Expr::col(entity::bids::Column::Amount).max(), "max_bid_user")
        .into_tuple::<(i32, Decimal)>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(json!({ "error": err.to_string() })))
        })?;

    let highest_bids_anyone = entity::bids::Entity::find()
        .group_by(entity::bids::Column::ListingId)
        .select_only()
        .column(entity::bids::Column::ListingId)
        .column_as(Expr::col(entity::bids::Column::Amount).max(), "max_bid_anyone")
        .into_tuple::<(i32, Decimal)>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(json!({ "error": err.to_string() })))
        })?;

    let total_bids = entity::bids::Entity::find()
        .group_by(entity::bids::Column::ListingId)
        .select_only()
        .column(entity::bids::Column::ListingId)
        .column_as(Expr::col(entity::bids::Column::Id).count(), "total_bids")
        .into_tuple::<(i32, i64)>()
        .all(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(json!({ "error": err.to_string() })))
        })?;

    let highest_user_bids_map: HashMap<i32, Decimal> = highest_user_bids.into_iter().collect();
    let highest_bids_anyone_map: HashMap<i32, Decimal> = highest_bids_anyone.into_iter().collect();
    let total_bids_map: HashMap<i32, i64> = total_bids.into_iter().collect();

    let auctions_data = active_auctions.into_iter().map(|(listing_id, start_time, end_time, title, description, base_price, available_volume)| {
        json!({
            "listing_id": listing_id,
            "end_time": end_time,
            "title": title,
            "description": description,
            "base_price": base_price,
            "available_volume": available_volume,
            "status": if start_time > now { "Not Started".to_string() } else { "Active".to_string() },
            "highest_user_bid": highest_user_bids_map.get(&listing_id).unwrap_or(&Decimal::ZERO),
            "highest_anyone_bid": highest_bids_anyone_map.get(&listing_id).unwrap_or(&Decimal::ZERO),
            "total_bids": total_bids_map.get(&listing_id).unwrap_or(&0),
        })
    }).collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(json!({
        "auctions": auctions_data,
        "message": "Auctions fetched successfully"
    })) ))
}
