use axum::routing::get;
use crate::AppState;
use crate::controllers::user;
use axum::Router;
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/activities", get(user::note_list_handler))
        .with_state(app_state)
}