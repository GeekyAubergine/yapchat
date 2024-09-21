use axum::{
    extract::Json,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;

use crate::{domain::models::User, infrastructure::appstate::AppState, ResponseResult};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
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

    // Set user uuid in session

    let mut headers = HeaderMap::new();
    headers.append(
        "Set-Cookie",
        format!("user_uuid={}; Path=/", user.uuid).parse().unwrap(),
    );
    headers.append("Set-Cookie",
        format!("user_name={}; Path=/api", user.uuid).parse().unwrap(),
    );
    headers.append("Set-Cookie",
        format!("user_name={}; Path=/chat", user.uuid).parse().unwrap(),
    );
    headers.append("HX-Redirect", "/chat".parse().unwrap());

    Ok((headers, "Logged in"))
}
