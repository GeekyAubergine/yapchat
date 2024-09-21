use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};

use crate::{
    domain::models::User, error::Error, infrastructure::appstate::AppState, ResponseResult,
};

pub trait FormatDate {
    fn short_iso(&self) -> String;
    fn datetime(&self) -> String;
    fn without_time(&self) -> String;
}

impl FormatDate for DateTime<Utc> {
    fn short_iso(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn datetime(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn without_time(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}

impl FormatDate for Option<DateTime<Utc>> {
    fn short_iso(&self) -> String {
        match self {
            Some(date) => date.short_iso(),
            None => "-".to_string(),
        }
    }

    fn datetime(&self) -> String {
        match self {
            Some(date) => date.datetime(),
            None => "-".to_string(),
        }
    }

    fn without_time(&self) -> String {
        match self {
            Some(date) => date.without_time(),
            None => "-".to_string(),
        }
    }
}

pub async fn validate_user(state: &AppState, jar: CookieJar) -> ResponseResult<User> {
    let user_id = jar
        .get("user_uuid")
        .and_then(|cookie| cookie.value().parse::<String>().ok());

    match user_id {
        Some(user_id) => {
            let user_id = uuid::Uuid::parse_str(&user_id).unwrap();
            let user = state.user_repo().find_user_by_uuid(&user_id).await?;
            match user {
                Some(user) => Ok(user),
                None => Err(Error::NotLoggedIn.into_response()),
            }
        }
        None => Err(Error::NotLoggedIn.into_response()),
    }
}
