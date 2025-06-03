use axum::response::{IntoResponse, Json, Response};
use http::StatusCode;
use serde_json::{json, Value};

use crate::auth::Claims;

/// Handler for the health check endpoint, just returns HTTP 200 "OK"
pub async fn healthz() -> &'static str {
    "OK"
}

pub enum MaybeJson {
    Json(Value),
    None,
}

/// Implement IntoResponse for MaybeJson
impl IntoResponse for MaybeJson {
    fn into_response(self) -> Response {
        match self {
            MaybeJson::Json(value) => Json(value).into_response(),
            MaybeJson::None => ().into_response(),
        }
    }
}

/// The authors endpoint
pub async fn authors(claims: Claims) -> (StatusCode, MaybeJson) {
    println!("{:#?}", claims);
    (StatusCode::OK, MaybeJson::Json(json!({})))
}
