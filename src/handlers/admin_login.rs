use serde::Deserialize;
use axum::extract::Json;
use std::env;
use argon2::{
    Argon2,
    PasswordHash,
    PasswordVerifier,
};
#[derive(Deserialize)]
pub struct Credential {
    email: String,
    password:  String
}

pub async fn admin_login(Json(body): Json<Credential>) -> &'static str {
    let admin_email =
        env::var("ADMIN_EMAIL").expect("Could not read ADMIN_EMAIL from env");

    let admin_password_hash =
        env::var("ADMIN_PASSWORD_HASH").expect("Could not read ADMIN_PASSWORD_HASH from env");

    if body.email != admin_email {
        return "unauthorized";
    }

    let parsed_hash =
        PasswordHash::new(&admin_password_hash).expect("Invalid hash");

    let is_correct: bool =
        Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash).is_ok();

    if !is_correct {"unauthorized";}

    return "0";
}
