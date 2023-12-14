use axum::Json;
use serde::Deserialize;
use serde_json::Value;

use crate::error::{Error, Result};

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    todo!()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
