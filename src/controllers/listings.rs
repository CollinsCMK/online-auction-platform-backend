use std::str::FromStr;

use actix_web::{delete, get, post, put, web};
use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set, ColumnTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingData {
    title: String,
    description: Option<String>,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    base_price: String,
    available_volume: Option<i32>,
}

impl ListingData {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title is required".to_string());
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

        if self.base_price.to_string().is_empty() {
            return Err("Base Price is required".to_string());
        }

        Ok(())
    }
}

#[post("/create")]
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
        start_time: Set(listing_data.start_time.clone()),
        end_time: Set(listing_data.end_time.clone()),
        base_price: Set(Decimal::from_str(&listing_data.base_price.clone()).expect("A decimal number")),
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

#[put("/update/{id}")]
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
    update_listing_model.description = Set(listing_data.description.clone());
    update_listing_model.start_time = Set(listing_data.start_time.clone());
    update_listing_model.end_time = Set(listing_data.end_time.clone());
    update_listing_model.base_price = Set(Decimal::from_str(&listing_data.base_price.clone()).expect("A decimal number"));
    update_listing_model.available_volume = Set(listing_data.available_volume.unwrap_or(1));
    update_listing_model.updated_at = Set(Utc::now().naive_local());
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

#[get("/listings/get")]
pub async fn get_all_listings(
    app_state: web::Data<AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let listing_model = entity::listings::Entity::find()
        .filter(entity::listings::Column::DeletedAt.is_null())
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
                "title": row.title,
                "description": row.description,
                "start_time": row.start_time,
                "end_time": row.end_time,
                "base_price": row.base_price,
                "available_volume": row.available_volume,
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