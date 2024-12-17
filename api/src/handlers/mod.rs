use abi::serde_json::{json, Value};

use axum::Json;

use serde::Serialize;

pub mod admin;
pub mod users;

pub fn json_helper<T: Serialize>(value: T) -> Json<Value> {
    return Json(json!({
        "code": 200,
        "data": value,
        "message": ""
    }));
}
