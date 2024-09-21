use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(sqlx::Error),

    #[error("Database query error: {0}")]
    QueryError(sqlx::Error),
}

impl DatabaseError {
    pub fn from_connection_error(err: sqlx::Error) -> Error {
        Error::DatabaseError(DatabaseError::ConnectionError(err))
    }

    pub fn from_query_error(err: sqlx::Error) -> Error {
        Error::DatabaseError(DatabaseError::QueryError(err))
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

impl Error {
    pub fn into_response(self) -> ErrorResponse {
        match self {
            Error::DatabaseError(err) => err.into_response(),
            _ => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal Server Error",
            },
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
