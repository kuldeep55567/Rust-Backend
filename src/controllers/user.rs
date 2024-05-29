use axum::{extract::{Query, State}, Json};
use axum::response::IntoResponse;
use crate::AppState;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Default, Deserialize)]
pub struct FilterOptions {
    pub limit: Option<usize>,
}

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    state: State<Arc<AppState>>,
) -> impl IntoResponse {
    let limit = opts.unwrap_or_default().limit.unwrap_or(10);
    match query_database(&state.db, limit).await {
        Ok(notes) => (
            axum::http::StatusCode::OK,
            Json(json!({ "status": "ok", "notes": notes })),
        )
            .into_response(),
        Err(e) => {
            let error_message = format!("Database error: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": error_message })),
            )
                .into_response()
        }
    }
}

async fn query_database(pool: &MySqlPool, limit: usize) -> Result<Vec<Note>, sqlx::Error> {
    sqlx::query_as!(Note, "SELECT id FROM app_activities ORDER BY id LIMIT ?", limit as i32)
        .fetch_all(pool)
        .await
}

#[derive(Serialize, sqlx::FromRow)]
struct Note {
    id: i32,
}