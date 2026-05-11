use axum::{
    routing::{post, delete},
    Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use std::env::var;
use dotenvy;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use crate::{db::run_migrations, handlers::{admin_login, registrations, checkin, projects}};
mod handlers;
mod models;
mod middleware;
mod db;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = 
        var("DATABASE_URL").expect("Could not read database URL.");

    let connection =
        SqlitePool::connect(&db_url)
            .await.expect("Failed while connecting to sqlite database");
    run_migrations(&connection)
        .await
        .expect("Failed while running migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/checkin", post(checkin::checkin))
        .route("/checkin/{ra}", delete(checkin::undo_checkin))
        .route("/registrations", post(registrations::register_student).get(registrations::list_registrations))
        .route("/registrations/{ra}", delete(registrations::delete_registration))
        .route("/projects", post(projects::create_project).get(projects::list_projects))
        .route("/admin/login", post(admin_login::admin_login))
        .layer(cors)
        .with_state(connection);

    let listener =
        TcpListener::bind("0.0.0.0:3000")
        .await.expect(
            "Failed while triying to establish port listener");

    axum::serve(listener, app).await.expect(
        "Failed while triying to build server.");

    
}
