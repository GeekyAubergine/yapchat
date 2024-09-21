use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl Error {
    pub fn into_response(self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }
}

pub struct ErrorResponse {
    pub status: StatusCode,
    pub message: &'static str,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        err.into_response()
    }
}

impl From<(reqwest::StatusCode, &'static str)> for ErrorResponse {
    fn from((status, message): (reqwest::StatusCode, &'static str)) -> Self {
        ErrorResponse { status, message }
    }
}
