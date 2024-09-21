use std::{
    borrow::Cow, net::SocketAddr, ops::ControlFlow, sync::Arc, thread::sleep, time::Duration,
};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use tokio::sync::Mutex;
use ws::ws_handler;

use crate::infrastructure::appstate::AppState;

pub mod api;
pub mod web;
pub mod ws;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::router())
        .nest("/", web::router())
        .route("/ws/:user_uuid", get(ws_handler))
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
