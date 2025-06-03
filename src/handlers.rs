/// The handlers for the routes

use axum::response::{IntoResponse, Json, Response};
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use http::StatusCode;
use serde_json::{Value, json};

/// Handler for the health check endpoint, just returns HTTP 200 "OK"
pub async fn healthz() -> &'static str {
    "OK"
}

pub enum MaybeJson {
    Json(Value),
    None
}

/// Implement IntoResponse for MaybeJson
impl IntoResponse for MaybeJson {
    fn into_response(self) -> Response {
        match self {
            MaybeJson::Json(value) => Json(value).into_response(),
            MaybeJson::None => ().into_response()
        }
    }
}


/// The authors endpoint
pub async fn authors(maybe_auth: Option<TypedHeader<Authorization<Bearer>>>) -> (StatusCode, MaybeJson) {
    println!("{:#?}", maybe_auth);
    if maybe_auth.is_none() {
        return (StatusCode::UNAUTHORIZED, MaybeJson::None);
    }
    if let TypedHeader(Authorization(bearer)) = maybe_auth.unwrap() {
        println!("bearer: {:?}", bearer)
    }
    (StatusCode::OK, MaybeJson::Json(json!({})))
}
