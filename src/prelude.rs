use crate::error::{Error, ErrorResponse};
use chrono::{DateTime, Utc};

use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

pub type Result<T> = std::result::Result<T, Error>;

pub type ResponseResult<T> = std::result::Result<T, ErrorResponse>;

pub type DatabaseConnection = Pool<Postgres>;
