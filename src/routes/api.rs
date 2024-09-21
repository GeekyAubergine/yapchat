use std::f32::consts::PI;

use askama::Template;
use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use axum_extra::extract::CookieJar;
use hound::WavWriter;
use serde::Deserialize;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;
use uuid::Uuid;

use crate::utils::{validate_user, FormatDate};

use crate::{
    domain::models::{ChatMessage, User},
    infrastructure::appstate::AppState,
    ResponseResult,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/chats/:chat_uuid/message", post(add_message))
        .route("/chats/:chat_uuid/audio", post(add_audio))
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> ResponseResult<impl IntoResponse> {
    let user = state
        .user_repo()
        .find_user_by_name(&payload.username)
        .await
        .unwrap();

    let user = match user {
        Some(user) => user,
        None => User::from_name(payload.username),
    };

    let user = state.user_repo().commit_user(user).await?.unwrap();

    // Set user uuid in session

    let mut headers = HeaderMap::new();
    headers.append(
        "Set-Cookie",
        format!("user_uuid={}; Path=/", user.uuid).parse().unwrap(),
    );
    headers.append(
        "Set-Cookie",
        format!("user_name={}; Path=/api", user.uuid)
            .parse()
            .unwrap(),
    );
    headers.append(
        "Set-Cookie",
        format!("user_name={}; Path=/chat", user.uuid)
            .parse()
            .unwrap(),
    );
    headers.append("HX-Redirect", "/chat".parse().unwrap());

    Ok((headers, "Logged in"))
}

#[derive(Template)]
#[template(path = "chat/chat_message_input.html")]
pub struct ChatMessageInputTemplate {
    chat_uuid: String,
}

#[derive(Template)]
#[template(path = "chat/chat_message.html")]
pub struct ChatMessageTemplate {
    message: ChatMessage,
}

#[derive(Template)]
#[template(path = "chat/chat_messages_list.html")]
pub struct ChatMessageListTemplate {
    chat_messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct AddMessagePayload {
    message: String,
}

async fn add_message(
    State(state): State<AppState>,
    Path(chat_uuid): Path<String>,
    jar: CookieJar,
    Json(payload): Json<AddMessagePayload>,
) -> ResponseResult<ChatMessageInputTemplate> {
    let user = validate_user(&state, jar).await?;

    let chat_uuid = Uuid::parse_str(&chat_uuid).unwrap();

    let message =
        ChatMessage::from_chat_uuid_sender_uuid_file_uuid(chat_uuid, user.uuid, Uuid::new_v4());

    let message = state.chat_message_repo().commit(message).await?.unwrap();

    // let value = ChatMessageTemplate { message };

    // let value = value.render().unwrap();

    // state.ws_clients().send(&user.uuid, (value)).await;
    info!("chat uuid {}", chat_uuid);

    let chat_messages = state
        .chat_message_repo()
        .find_chat_messages_in_chat_ordered_by_created_at(&chat_uuid)
        .await?;

    info!("Chat messages {}", chat_messages.len());

    let value = ChatMessageListTemplate { chat_messages };

    let value = value.render().unwrap();

    state.ws_clients().send(&user.uuid, (value)).await;

    Ok(ChatMessageInputTemplate {
        chat_uuid: chat_uuid.to_string(),
    })
}

#[derive(Deserialize)]
struct ChatAudioPayload {
    audio: Vec<u8>,
}

async fn add_audio(
    State(state): State<AppState>,
    Path(chat_uuid): Path<String>,
    jar: CookieJar,
    Json(payload): Json<ChatAudioPayload>,
) -> ResponseResult<()> {
    let user = validate_user(&state, jar).await?;

    let chat_uuid = Uuid::parse_str(&chat_uuid).unwrap();

    let message =
        ChatMessage::from_chat_uuid_sender_uuid_file_uuid(chat_uuid, user.uuid, Uuid::new_v4());

    let message = state.chat_message_repo().commit(message).await?.unwrap();

    let file_uuid = message.file_uuid;

    let mut writer = WavWriter::create(
        format!("cdn/{}.wav", file_uuid.to_string()),
        hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        },
    )
    .unwrap();

    let data = payload.audio;

    let data = &data[88..data.len()];

    // Convert u8 array to i16 array
    let normalised = data
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let mut bytes = [0u8; 2];
            for (i, byte) in chunk.iter().enumerate() {
                bytes[i] = *byte;
            }
            i16::from_le_bytes(bytes)
        })
        .collect::<Vec<i16>>();


    for sample in normalised {
        writer.write_sample(sample).unwrap();
    }

    info!("Audio written");

    // let value = ChatMessageTemplate { message };

    // let value = value.render().unwrap();

    // state.ws_clients().send(&user.uuid, (value)).await;
    info!("chat uuid {}", chat_uuid);

    let chat_messages = state
        .chat_message_repo()
        .find_chat_messages_in_chat_ordered_by_created_at(&chat_uuid)
        .await?;

    info!("Chat messages {}", chat_messages.len());

    let value = ChatMessageListTemplate { chat_messages };

    let value = value.render().unwrap();

    state.ws_clients().send(&user.uuid, (value)).await;

    Ok(())
}
