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

#[derive(Serialize, Clone)]
struct Category {
    activity_type: Option<i32>,
    activities: Vec<Note>,
}

#[derive(Serialize, Clone)]
struct Coach {
    id: Option<i32>,
    name: Option<String>
}


pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    state: State<Arc<AppState>>,
) -> impl IntoResponse {
    let limit = opts.unwrap_or_default().limit.unwrap_or(100);
    match query_database(&state.db, limit).await {
        Ok(notes) => {
            let mut categories: Vec<Category> = Vec::new();
            for note in &notes {
                let activity_type = note.activity_type.clone();
                if let Some(category) = categories.iter_mut().find(|c| c.activity_type == activity_type) {
                    category.activities.push(note.clone());
                } else {
                    categories.push(Category {
                        activity_type,
                        activities: vec![note.clone()],
                    });
                }
            }

            let json_response = json!({ "status": "ok", "categories": categories });
            (axum::http::StatusCode::OK, Json(json_response)).into_response()
        }
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
    sqlx::query_as!(
        Note,
        r#"
        SELECT 
            id, 
            name, 
            description, 
            category, 
            activity_type, 
            series_id, 
            xp
        FROM 
            app_activities
        ORDER BY 
            id
        LIMIT ?
        "#,
        limit as i32
    )
    .fetch_all(pool)
    .await
}

pub async fn coaches_list_handler(
    state: State<Arc<AppState>>,
) -> impl IntoResponse {
    match query_coach_database(&state.db).await {
        Ok(coaches) => {
            let json_response = json!({ "status": "ok", "coaches": coaches });
            (axum::http::StatusCode::OK, Json(json_response)).into_response()
        }
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
async fn query_coach_database(pool: &MySqlPool) -> Result<Vec<Coach>,sqlx::Error>{
    sqlx::query_as!(
        Coach,
        r#"
        SELECT 
            id, 
            name
        FROM 
            coaches
        "#
    )
    .fetch_all(pool)
    .await
}

#[derive(Serialize, Clone, sqlx::FromRow)]
struct Note {
    id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    category: Option<String>,
    activity_type: Option<i32>,
    series_id: Option<i32>,
    xp: Option<i32>,
}
