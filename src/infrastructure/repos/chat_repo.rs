use std::collections::HashMap;

use uuid::Uuid;

use crate::{domain::models::Chat, error::DatabaseError, prelude::*};

#[derive(Debug, Clone)]
pub struct ChatRepo {
    database_connection: DatabaseConnection,
}

impl ChatRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_chat_by_uuid(&self, uuid: Uuid) -> Result<Option<Chat>> {
        let row = sqlx::query!(
            "
            SELECT
            chats.uuid as uuid,
            chats.name as name,
            chats.created_at as created_at,
            chats.updated_at as updated_at,
            chats.deleted_at as deleted_at,
            (
                SELECT
                created_at
                FROM chat_messages
                WHERE chat_uuid = chats.uuid
                ORDER BY created_at DESC
                LIMIT 1
            ) as latest_message_created_at,
            array(
                SELECT
                users.user_name
                FROM users
                JOIN users_in_chat ON users_in_chat.user_uuid = users.uuid
                WHERE users_in_chat.chat_uuid = chats.uuid
            ) as user_names
            FROM chats
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        match row {
            None => return Ok(None),
            Some(row) => {
                return Ok(Some(Chat {
                    uuid: row.uuid,
                    name: row.name,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    deleted_at: row.deleted_at,
                    latest_message_created_at: row.latest_message_created_at,
                    user_names: match row.user_names {
                        Some(user_names) => user_names,
                        None => Vec::new(),
                    },
                }))
            }
        }
    }

    pub async fn find_all_ordered_by_latest_message(&self) -> Result<Vec<Chat>> {
        let rows = sqlx::query!(
            "
            SELECT
            chats.uuid as uuid,
            chats.name as name,
            chats.created_at as created_at,
            chats.updated_at as updated_at,
            chats.deleted_at as deleted_at,
            (
                SELECT
                created_at
                FROM chat_messages
                WHERE chat_uuid = chats.uuid
                ORDER BY created_at DESC
                LIMIT 1
            ) as latest_message_created_at,
            array(
                SELECT
                users.user_name
                FROM users
                JOIN users_in_chat ON users_in_chat.user_uuid = users.uuid
                WHERE users_in_chat.chat_uuid = chats.uuid
            ) as user_names
            FROM chats
            ORDER BY latest_message_created_at DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows
            .into_iter()
            .map(|row| Chat {
                uuid: row.uuid,
                name: row.name,
                created_at: row.created_at,
                updated_at: row.updated_at,
                deleted_at: row.deleted_at,
                latest_message_created_at: row.latest_message_created_at,
                user_names: match row.user_names {
                    Some(user_names) => user_names,
                    None => Vec::new(),
                },
            })
            .collect())
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
