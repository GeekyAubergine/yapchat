use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    domain::models::{ChatMetadata, ChatWithMetadata},
    infrastructure::repos::{
        chat_message_repo::ChatMessageRepo,
        chat_repo::ChatRepo,
        chat_users_repo::{self, ChatUsersRepo}, user_repo::UsersRepo,
    },
    Result,
};

pub struct ChatQueryService;

impl ChatQueryService {
    pub async fn find_chat(
        chat_repo: &ChatRepo,
        users_repo: &UsersRepo,
        chat_messages_repo: &ChatMessageRepo,
    ) -> Result<ChatWithMetadata> {
        unimplemented!()
    }

    pub async fn find_all_chats_ordered_by_last_message(
        chat_repo: &ChatRepo,
        chat_messages_repo: &ChatMessageRepo,
        chat_users_repo: &ChatUsersRepo,
    ) -> Result<HashMap<Uuid, ChatWithMetadata>> {
        let chats = chat_repo.find_all_chats().await?;

        let mut chat_with_metadata = HashMap::new();

        for (chat_uuid, chat) in chats.iter() {
            let most_recent_message = chat_messages_repo
                .find_most_recent_chat_message_by_chat_uuid(chat_uuid)
                .await?;

            let users = chat_users_repo.find_users_by_chat_uuid(chat_uuid).await?;

            chat_with_metadata.insert(
                chat_uuid.clone(),
                ChatWithMetadata {
                    chat: chat.clone(),
                    metadata: ChatMetadata {
                        users_names: users.values().map(|user| user.user_name.clone()).collect(),
                        last_message: most_recent_message.map(|message| message.created_at.clone()),
                    },
                },
            );
        }

        Ok(chat_with_metadata)
    }
}
