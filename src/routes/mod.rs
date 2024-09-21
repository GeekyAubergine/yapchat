use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::infrastructure::appstate::AppState;

pub mod api;
pub mod web;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::router())
        .nest("/", web::router())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
