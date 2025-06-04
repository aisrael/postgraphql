use crate::app_state::AppState;
use crate::auth::Claims;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Json, Response};
use http::StatusCode;
use serde_json::{json, Value};

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
pub async fn authors(State(state): State<AppState>, claims: Claims) -> (StatusCode, MaybeJson) {
    println!("{:?}", state);
    println!("{:?}", claims);
    let AppState { book_store } = state;
    match book_store.list_authors().await {
        Ok(authors) => (StatusCode::OK, MaybeJson::Json(json!(authors))),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, MaybeJson::None)
        }
    }
}

pub async fn get_author_by_id(
    State(state): State<AppState>,
    _claims: Claims,
    Path(author_id): Path<i64>,
) -> (StatusCode, MaybeJson) {
    println!("author_id: {}", author_id);

    let AppState { book_store } = state;
    match book_store.find_author_by_id(author_id).await {
        Ok(maybe_author) => match maybe_author {
            Some(author) => (StatusCode::OK, MaybeJson::Json(json!(author))),
            None => (StatusCode::NOT_FOUND, MaybeJson::None),
        },
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, MaybeJson::None)
        }
    }
}

pub async fn list_books_by_author_id(
    State(state): State<AppState>,
    _claims: Claims,
    Path(author_id): Path<i64>,
) -> (StatusCode, MaybeJson) {
    let AppState { book_store } = state;
    match book_store.list_books_by_author_id(author_id).await {
        Ok(books) => (StatusCode::OK, MaybeJson::Json(json!(books))),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, MaybeJson::None)
        }
    }
}
