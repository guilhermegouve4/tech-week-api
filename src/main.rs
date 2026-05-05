use axum::{
    routing::get,
    routing::post,
    Router,
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use dotenvy;
use crate::{db::run_migrations, handlers::admin_login};
mod handlers;
mod models;
mod db;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let connection =
        SqlitePool::connect("sqlite:db/weektech.db?mode=rwc")
            .await.expect("Failed while connecting to sqlite database");
    run_migrations(&connection)
        .await
        .expect("Failed while running migrations");

    let app = Router::new()
        .route("/registrations", get(handler))
        .route("/registrations", post(handler))
        .route("/admin/login", post(admin_login::admin_login))
        .with_state(connection);

    let listener =
        TcpListener::bind("0.0.0.0:3000")
        .await.expect(
            "Failed while triying to establish port listener");

    axum::serve(listener, app).await.expect(
        "Failed while triying to build server.");

    
}

async fn handler() -> &'static str {
    "ok"
}
