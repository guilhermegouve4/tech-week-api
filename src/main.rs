use axum::{
    routing:: get,
    routing:: post,
    Router,
};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {

    let connection =
        SqlitePool::connect("sqlite:db/weektech.db?mode=rwc")
            .await.expect("Failed while connecting to sqlite database");

    let app = Router::new()
        .route("/register", get(handler))
        .route("/register", post(handler))
        .with_state(connection);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.expect(
            "Failed while triying to establish port listener");

    axum::serve(listener, app).await.expect(
        "Failed while triying to build server.");

    
}

async fn handler() -> &'static str {
    "ok"
}
