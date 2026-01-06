use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};

pub async fn get_health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": Status::Success })))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Error,
    Success,
}
