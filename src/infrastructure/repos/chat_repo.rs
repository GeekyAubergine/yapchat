use std::collections::HashMap;

use uuid::Uuid;

use crate::{domain::models::Chat, error::DatabaseError, prelude::*};

pub struct ChatRepo {
    database_connection: DatabaseConnection,
}

impl ChatRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all_chats(&self) -> Result<HashMap<Uuid, Chat>> {
        let rows = sqlx::query_as!(
            Chat,
            "
            SELECT *
            FROM chats
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().map(|chat| (chat.uuid, chat)).collect())
    }

    pub async fn find_chat_by_uuid(&self, uuid: Uuid) -> Result<Option<Chat>> {
        let row = sqlx::query_as!(
            Chat,
            "
            SELECT *
            FROM chats
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row)
    }

    pub async fn commit_chat(&self, chat: Chat) -> Result<Option<Chat>> {
        if let Some(_) = self.find_chat_by_uuid(chat.uuid).await? {
            sqlx::query!(
                "
                UPDATE chats
                SET name = $2, updated_at = NOW(), deleted_at = $3
                WHERE uuid = $1
                ",
                chat.uuid,
                chat.name,
                chat.deleted_at
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return self.find_chat_by_uuid(chat.uuid).await;
        }

        sqlx::query!(
            "
            INSERT INTO chats (uuid, name, created_at, updated_at, deleted_at)
            VALUES ($1, $2, NOW(), NOW(), $3)
            ",
            chat.uuid,
            chat.name,
            chat.deleted_at
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        return self.find_chat_by_uuid(chat.uuid).await;
    }
}
