use actix_web::{delete, get, post, web::{self}};
use chrono::Utc;
use regex::Regex;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{api_response::ApiResponse, app_state::AppState, json_response::response};

#[get("/user/get/{phone_number}")]
pub async fn get_user(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<ApiResponse, ApiResponse> {
    let phone_number = path.into_inner();

    let users_model = entity::users::Entity::find()
        .filter(entity::users::Column::DeletedAt.is_null())
        .filter(entity::users::Column::PhoneNumber.eq(phone_number))
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
                    "error": "User not found.".to_string()
                })
            ))
        })?;

    Ok(ApiResponse::new(200, response(
        json!({
            "phone_number": users_model.phone_number,
            "name": users_model.name,
            "message": "User data retrieved successfully".to_string()
        })
    )))
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    phone_number: String,
    name: String,
}

impl UserData {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.phone_number.is_empty() {
            return Err("Phone number is required".to_string());
        }

        if !Self::validate_phone_number(&self.phone_number) {
            return Err("Phone number is not valid.".to_string());
        }

        Ok(())
    }

    fn validate_phone_number(phone_number: &str) -> bool {
        let phone_regex = Regex::new(r"^\+254\d{9}$").unwrap();
        phone_regex.is_match(phone_number)
    }
}

#[post("/user/create")]
pub async fn create_user(
    app_state: web::Data<AppState>,
    user_data: web::Json<UserData>
) -> Result<ApiResponse, ApiResponse> {
    if let Err(err) = user_data.validate() {
        return Err(ApiResponse::new(500, response(
            json!({
                "error": err.to_string()
            })
        )));
    }

    let user_model = entity::users::Entity::find()
        .filter(entity::users::Column::DeletedAt.is_null())
        .filter(entity::users::Column::PhoneNumber.eq(user_data.phone_number.clone()))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            ApiResponse::new(500, response(
                json!({
                    "error": err.to_string()
                })
            ))
        })?;

    if let Some(user) = user_model {
        let mut update_user_model: entity::users::ActiveModel = user.to_owned().into_active_model();
        update_user_model.name = Set(user_data.name.clone());
        update_user_model.phone_number = Set(user_data.phone_number.clone());
        if user.name.clone() != user_data.name && user.phone_number.clone() != user_data.phone_number {
            update_user_model.updated_at = Set(Utc::now().naive_local());
        } 
        update_user_model
            .update(&app_state.db)
            .await
            .map_err(|err| {
                ApiResponse::new(500, response(
                    json!({
                        "error": err.to_string()
                    })
                ))
            })?;

        if user.name.clone() != user_data.name && user.phone_number.clone() != user_data.phone_number {
            return Ok(ApiResponse::new(200, response(
                json!({
                    "message": "User updated successfully. Continue to auction".to_string()
                })
            )));
        }

        return Ok(ApiResponse::new(200, response(
            json!({
                "message": "User exists, continue to auction".to_string()
            })
        )));
    }

    entity::users::ActiveModel {
        name: Set(user_data.name.clone()),
        phone_number: Set(user_data.phone_number.clone()),
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
            "message": "User created successfully".to_string()
        })
    )))
}

#[get("/users/get")]
pub async fn get_all_users(
    app_state: web::Data<AppState>
) -> Result<ApiResponse, ApiResponse> {
    let users_model = entity::users::Entity::find()
        .filter(entity::users::Column::DeletedAt.is_null())
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
                "phone_number": row.phone_number,
                "updated_at": row.updated_at,
            })
        })
        .collect::<Vec<_>>();

    Ok(ApiResponse::new(200, response(
        json!({
            "users": users_model,
            "message": "Users fetched successfully".to_string()
        })
    )))
}

#[delete("/user/delete/{id}")]
pub async fn delete_user(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<ApiResponse, ApiResponse> {
    let user_id = path.into_inner();

    let _users_model = entity::users::Entity::delete_by_id(user_id)
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
            "message": "User deleted successfully".to_string()
        })
    )))
} 