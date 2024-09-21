use askama::Template;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json,
    Router,
};

use crate::prelude::*;
use crate::{get_build_date, infrastructure::appstate::AppState, ResponseResult};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(chat))
}

#[derive(Template)]
#[template(path = "chat.html")]
pub struct ChatTemplate {
    page_title: String,
    page_description: String,
    build_date: String,
}

async fn chat(State(sate): State<AppState>) -> ResponseResult<ChatTemplate> {
    Ok(ChatTemplate {
        page_title: "Chat".to_string(),
        page_description: "Chat with me".to_string(),
        build_date: get_build_date(),
    })
}
