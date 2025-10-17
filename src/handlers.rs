use actix_web::{Responder, get, http::StatusCode};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::Response;

#[get("/health")]
pub async fn health_handler() -> impl Responder {
    Response::create_json_response(true, "system was healthy", StatusCode::OK)
}

#[get("/server_time")]
pub async fn server_time_handler() -> impl Responder {
    let server_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Response::create_json_response(true, server_time, StatusCode::OK)
}
