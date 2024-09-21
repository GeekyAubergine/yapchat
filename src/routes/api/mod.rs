use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::infrastructure::appstate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}
