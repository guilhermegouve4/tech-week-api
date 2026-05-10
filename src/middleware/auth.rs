use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}, response::{IntoResponse, Json, Response}};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::env::var;
use crate::models::claims::Claims;

pub struct AuthGuard;

impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "missing_token"}))).into_response()),
        };

        let secret = var("JWT_SECRET").expect("Could not read JWT_SECRET from env");
        let key = DecodingKey::from_secret(secret.as_bytes());

        match decode::<Claims>(token, &key, &Validation::default()) {
            Ok(_) => Ok(AuthGuard),
            Err(_) => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "invalid_token"}))).into_response()),
        }
    }
}
