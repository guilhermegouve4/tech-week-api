use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use sqlx::SqlitePool;
use serde_json::json;
use crate::models::project::{Project, ProjectRow};
use crate::middleware::auth::AuthGuard;

pub async fn list_projects(
    _guard: AuthGuard,
    State(pool): State<SqlitePool>,
) -> Response {
    let result = sqlx::query_as::<_, ProjectRow>(
        "SELECT id, submitter_name, submitter_registration, project_name, description FROM projects"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => (StatusCode::OK, Json(rows)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(body): Json<Project>,
) -> Response {
    if let Err(msg) = body.validate_project() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response();
    }

    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM registrations WHERE student_registration = ?"
    )
    .bind(&body.submitter_registration)
    .fetch_one(&pool)
    .await;

    match exists {
        Ok(count) if count == 0 =>
            return (StatusCode::BAD_REQUEST, Json(json!({"error": "ra_not_found"}))).into_response(),
        Err(_) =>
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
        _ => {}
    }

    let result = sqlx::query(
        "INSERT INTO projects (submitter_name, submitter_registration, project_name, description)
         VALUES (?, ?, ?, ?)"
    )
    .bind(&body.submitter_name)
    .bind(&body.submitter_registration)
    .bind(&body.project_name)
    .bind(&body.description)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "project_created"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "db_error"}))).into_response(),
    }
}
