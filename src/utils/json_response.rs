use serde_json::Value;

pub fn response(response: Value) -> String {

    let json_response = serde_json::to_string(&response).unwrap();

    json_response
}