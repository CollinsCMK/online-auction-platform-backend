use serde_json::json;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Response};

use super::{api_response::ApiResponse, json_response::response};

pub async fn send_whatsapp_message(
    phone_number: &str,
    message: &str,
) -> Result<Response, ApiResponse> {    
    let url = "https://graph.facebook.com/v22.0/62077h2474447573/messages";
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer EAAZAgCySNoHABOZCDM0pKAGSIb3LzLcdMJSHFzZAkH91MhYMzuBbujsfiPbI5R2gkmbSXob9XEYjRAheadGSblBhDgvVtKfajknZAJ7W0l3vaOTuFPZCPi6JlRIf5FLwVbRxBJ8CflHVingBiidfdatLZCZAo3fsiZAv2AaWseK7iJQpEmkOOAzA3qFM6e1cwdvVLxNzQwEZCVXeAjx5ZAGZAhrDZCZBaUXuzBx1ZBNXUZD"));
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
