use actix_web::{Responder, http::StatusCode, web::Json};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Response<T: Serialize> {
    is_ok: bool,
    body: T,
}

impl<T: Serialize> Response<T> {
    pub fn create_json_response(is_ok: bool, body: T, status_code: StatusCode) -> impl Responder {
        (
            Json(Self {
                is_ok: is_ok,
                body: body,
            }),
            status_code,
        )
    }
}
