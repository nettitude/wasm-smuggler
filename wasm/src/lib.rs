use std::str;
use wasm_bindgen::prelude::*;

// "myusername"
static USERNAME: &str = "bXl1c2VybmFtZQ==";
// "mypassword"
static PASSWORD: &str = "bXlwYXNzd29yZA==";
// The payload
static PAYLOAD: &str = "R290ZW0hCg==";

#[wasm_bindgen]
pub async fn validate_user(u: &str, p: &str) -> String {
    let username_vec: Vec<u8> = base64::decode(USERNAME).expect("Failed to decode");
    let username_string: String = str::from_utf8(&username_vec).unwrap().to_string();
    let password_vec: Vec<u8> = base64::decode(PASSWORD).expect("Failed to decode");
    let password_string: String = str::from_utf8(&password_vec).unwrap().to_string();

    if u != username_string || p != password_string {
        return "".to_string();
    }

    return PAYLOAD.to_string();
}
