use std::sync::Arc;

use crate::DatabaseConnection;

use super::repos::{
    chat_message_repo::ChatMessageRepo, chat_repo::ChatRepo, chat_users_repo::ChatUsersRepo,
    user_repo::UsersRepo,
};

#[derive(Debug, Clone)]
pub struct AppStateData {
    chat_repo: ChatRepo,
    chat_message_repo: ChatMessageRepo,
    chat_users_repo: ChatUsersRepo,
    user_repo: UsersRepo,
}

impl AppStateData {
    pub async fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            chat_repo: ChatRepo::new(database_connection.clone()),
            chat_message_repo: ChatMessageRepo::new(database_connection.clone()),
            chat_users_repo: ChatUsersRepo::new(database_connection.clone()),
            user_repo: UsersRepo::new(database_connection.clone()),
        }
    }

    pub fn chat_repo(&self) -> &ChatRepo {
        &self.chat_repo
    }

    pub fn chat_message_repo(&self) -> &ChatMessageRepo {
        &self.chat_message_repo
    }

    pub fn chat_users_repo(&self) -> &ChatUsersRepo {
        &self.chat_users_repo
    }

    pub fn user_repo(&self) -> &UsersRepo {
        &self.user_repo
    }
}

pub type AppState = Arc<AppStateData>;
