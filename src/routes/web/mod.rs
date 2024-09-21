use askama::Template;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::get, routing::post, Json,
    Router,
};

use crate::domain::models::{Chat, ChatMessage};
use crate::prelude::*;
use crate::utils::FormatDate;
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
    chats: Vec<Chat>,
    chat_messages: Vec<ChatMessage>,
}

async fn get_chat_messages_for_chat(
    state: &AppState,
    chat: Option<Chat>,
) -> Result<Vec<ChatMessage>> {
    match chat {
        Some(chat) => {
            state
                .chat_message_repo()
                .find_chat_messages_in_chat_ordered_by_created_at(&chat.uuid)
                .await
        }
        None => Ok(vec![]),
    }
}

async fn chat(State(state): State<AppState>) -> ResponseResult<ChatTemplate> {
    let chats = state
        .chat_repo()
        .find_all_ordered_by_latest_message()
        .await?;

    let first_chat = chats.first().cloned();

    let messages = get_chat_messages_for_chat(&state, first_chat).await?;

    Ok(ChatTemplate {
        page_title: "Chat".to_string(),
        page_description: "Chat with me".to_string(),
        build_date: get_build_date(),
        chats,
        chat_messages: messages,
    })
}
