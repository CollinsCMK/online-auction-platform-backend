use serde_json::json;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Response};

use super::{api_response::ApiResponse, constants, json_response::response};

pub async fn send_whatsapp_message(
    phone_number: &str,
    message: &str,
) -> Result<Response, ApiResponse> {  
    let whatsapp_phone_number_id = constants::WHATSAPP_PHONE_NUMBER_ID.to_string();  
    let url = &format!("https://graph.facebook.com/v22.0/{}/messages", whatsapp_phone_number_id);
    let mut headers = HeaderMap::new();
    let token = constants::WHATSAPP_ACCESS_TOKEN.to_string();
    
    // Use HeaderValue::from_str instead of from_static
    let auth_header = format!("Bearer {}", token);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = json!({
        "messaging_product": "whatsapp",
        "to": phone_number,
        "type": "text",
        "text": {
            "body": message
        }
    });

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|err| ApiResponse::new(500, response(
            json!({
                "error": err.to_string()
            })
        )))?;

    Ok(res)
}
