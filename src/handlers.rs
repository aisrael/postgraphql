use axum::extract::FromRequestParts;
use axum::response::{IntoResponse, Json, Response};
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use http::request::Parts;
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;
static DECODING_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set!");
    DecodingKey::from_secret(secret.as_bytes())
});

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidCredentials,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_message = match self {
            AuthError::MissingCredentials => "Missing credentials",
            AuthError::InvalidCredentials => "Invalid credentials",
            AuthError::InvalidToken => "Invalid token",
        };
        (StatusCode::UNAUTHORIZED, error_message).into_response()
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &DECODING_KEY, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, Header};

    #[test]
    pub fn test_keys() -> Result<(), anyhow::Error> {
        let header = Header::default();
        let claims = Claims {
            iss: "postgraphql".to_string(),
            sub: "aisrael".to_string(),
            exp: 1767225600,
        };
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set!");
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let encoded = encode::<Claims>(&header, &claims, &encoding_key)?;
        println!("{:?}", encoded);

        let decoded = decode::<Claims>(&encoded, &DECODING_KEY, &Validation::default())?;

        println!("{:?}", decoded);
        assert_eq!("postgraphql", decoded.claims.iss);
        assert_eq!("aisrael", decoded.claims.sub);

        Ok(())
    }
}
