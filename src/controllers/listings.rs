use actix_web::{delete, get, post, put, web};
use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[derive(Debug, Serialize, Deserialize)]
struct ListingData {
    title: String,
    description: Option<String>,
    base_price: Decimal,
    available_volume: Option<i32>,
    auction_id: i32,
}

impl ListingData {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title is required".to_string());
        }

        if self.base_price <= Decimal::ZERO {
            return Err("Base price must be greater than zero".to_string());
        }

        if self.auction_id.to_string().is_empty() {
            return Err("Auction ID is required".to_string());
        }

        Ok(())
    }
}

#[post("/listing/create")]
pub async fn create_listing(
    listing_data: web::Json<ListingData>,
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    if let Err(err) = listing_data.validate() {
        return Err(ApiResponse::new(500, response(
            json!({
                "error": err.to_string()
            })
        )));
    }

    entity::listings::ActiveModel {
        title: Set(listing_data.title.clone()),
        description: Set(listing_data.description.clone()),
        auction_id: Set(listing_data.auction_id.clone()),
        base_price: Set(listing_data.base_price.clone()),
        available_volume: Set(listing_data.available_volume.unwrap_or(1)),
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
            "message": "Listing created successfully".to_string()
        })
    )))
}

#[put("/listing/update/{id}")]
pub async fn update_listing(
    listing_data: web::Json<ListingData>,
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let listing_id = path.into_inner();

    let listing_model = entity::listings::Entity::find_by_id(listing_id)
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
                    "error": "Listing not found"
                })
            ))
        })?;

    let mut update_listing_model: entity::listings::ActiveModel = listing_model.to_owned().into();
    update_listing_model.title = Set(listing_data.title.clone());
    update_listing_model.auction_id = Set(listing_data.auction_id.clone());
    update_listing_model.description = Set(listing_data.description.clone());
    update_listing_model.base_price = Set(listing_data.base_price.clone());
    update_listing_model.available_volume = Set(listing_data.available_volume.unwrap_or(1));
    update_listing_model.updated_at = Set(Utc::now().naive_utc());
    update_listing_model
        .update(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error" : err.to_string()
                })
            ))
        })?;

    Ok(ApiResponse::new(200, response(
        json!({
            "message": "Listing updated successfully".to_string()
        })
    )))
}

#[derive(Debug, FromQueryResult)]
struct  AuctionResult {
    name: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    id: i32,
    title: String,
    description: Option<String>,
    base_price: Decimal,
    available_volume: i32,
    updated_at: NaiveDateTime,
}

#[get("/listings/get/{id}")]
pub async fn get_all_listings(
    path: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let auction_id = path.into_inner();

    let listing_model = entity::listings::Entity::find()
        .inner_join(entity::auctions::Entity)
        .filter(entity::auctions::Column::Id.eq(auction_id))
        .filter(entity::listings::Column::DeletedAt.is_null())
        .select_only()
        .column(entity::auctions::Column::Name)
        .column(entity::auctions::Column::StartTime)
        .column(entity::auctions::Column::EndTime)
        .column(entity::listings::Column::Id)
        .column(entity::listings::Column::Title)
        .column(entity::listings::Column::Description)
        .column(entity::listings::Column::BasePrice)
        .column(entity::listings::Column::AvailableVolume)
        .column(entity::listings::Column::UpdatedAt)
        .into_model::<AuctionResult>()
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
                "title": row.title,
                "description": row.description,
                "base_price": row.base_price,
                "available_volume": row.available_volume,
                "auction_name": row.name,
                "start_time": row.start_time,
                "end_time": row.end_time,
                "updated_at": row.updated_at,
            })
        })
        .collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(
        json!({
            "listings": listing_model,
            "message": "Listings fetched successfully".to_string()
        })
    )))
}

#[delete("/listing/delete/{id}")]
pub async fn delete_listing(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let listing_id = path.into_inner();

    let _listing_model = entity::listings::Entity::delete_by_id(listing_id)
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
            "message": "Listing deleted successfully".to_string()
        })
    )))
}