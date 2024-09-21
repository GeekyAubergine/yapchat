use std::collections::HashMap;

use uuid::Uuid;

use crate::{domain::models::ChatMessage, error::DatabaseError, DatabaseConnection, Result};

pub struct ChatMessageRepo {
    database_connection: DatabaseConnection,
}

impl ChatMessageRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all_chat_messages(&self) -> Result<HashMap<Uuid, ChatMessage>> {
        let rows = sqlx::query_as!(
            ChatMessage,
            "
            SELECT *
            FROM chat_messages
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows
            .into_iter()
            .map(|chat_message| (chat_message.uuid, chat_message))
            .collect())
    }

    pub async fn find_chat_message_by_uuid(&self, uuid: &Uuid) -> Result<Option<ChatMessage>> {
        let row = sqlx::query_as!(
            ChatMessage,
            "
            SELECT *
            FROM chat_messages
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row)
    }

    pub async fn find_chat_messages_by_chat_uuid(
        &self,
        chat_uuid: &Uuid,
    ) -> Result<HashMap<Uuid, ChatMessage>> {
        let rows = sqlx::query_as!(
            ChatMessage,
            "
            SELECT *
            FROM chat_messages
            WHERE chat_uuid = $1
            ",
            chat_uuid
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows
            .into_iter()
            .map(|chat_message| (chat_message.uuid, chat_message))
            .collect())
    }

    pub async fn find_most_recent_chat_message_by_chat_uuid(
        &self,
        chat_uuid: &Uuid,
    ) -> Result<Option<ChatMessage>> {
        let row = sqlx::query_as!(
            ChatMessage,
            "
            SELECT *
            FROM chat_messages
            WHERE chat_uuid = $1
            ORDER BY created_at DESC
            LIMIT 1
            ",
            chat_uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row)
    }

    pub async fn commit_chat_message(
        &self,
        chat_message: ChatMessage,
    ) -> Result<Option<ChatMessage>> {
        if let Some(_) = self.find_chat_message_by_uuid(&chat_message.uuid).await? {
            sqlx::query!(
                "
                UPDATE chat_messages
                SET
                    chat_uuid = $1,
                    sender_uuid = $2,
                    file_uuid = $3,
                    updated_at = NOW()
                WHERE uuid = $4
                ",
                chat_message.chat_uuid,
                chat_message.sender_uuid,
                chat_message.file_uuid,
                chat_message.uuid
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;
        } else {
            sqlx::query!(
                "
                INSERT INTO chat_messages (uuid, chat_uuid, sender_uuid, file_uuid, created_at, updated_at, deleted_at)
                VALUES ($1, $2, $3, $4, NOW(), NOW(), $5)
                ",
                chat_message.uuid,
                chat_message.chat_uuid,
                chat_message.sender_uuid,
                chat_message.file_uuid,
                chat_message.deleted_at
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;
        }

        Ok(Some(chat_message))
    }
}
