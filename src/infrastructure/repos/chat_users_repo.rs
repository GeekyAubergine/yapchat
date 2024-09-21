use std::collections::HashMap;

use uuid::Uuid;

use crate::prelude::*;
use crate::{domain::models::User, error::DatabaseError};

#[derive(Debug, Clone)]
pub struct ChatUsersRepo {
    database_connection: DatabaseConnection,
}

impl ChatUsersRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    // pub async fn find_users_by_chat_uuid(&self, chat_uuid: &Uuid) -> Result<HashMap<Uuid, User>> {
    //     let rows = sqlx::query_as!(
    //         User,
    //         "
    //         SELECT users.*
    //         FROM users
    //         JOIN users_in_chat
    //         ON users.uuid = users_in_chat.user_uuid
    //         WHERE users_in_chat.chat_uuid = $1
    //         ",
    //         chat_uuid
    //     )
    //     .fetch_all(&self.database_connection)
    //     .await
    //     .map_err(DatabaseError::from_query_error)?;

    //     Ok(rows.into_iter().map(|user| (user.uuid, user)).collect())
    // }

    // pub async fn commit_user_to_chat(&self, chat_uuid: &Uuid, user: User) -> Result<()> {
    //     if let Some(_) = self
    //         .find_users_by_chat_uuid(chat_uuid)
    //         .await?
    //         .get(&user.uuid)
    //     {
    //         sqlx::query!(
    //             "
    //             INSERT INTO users_in_chat (chat_uuid, user_uuid, updated_at)
    //             VALUES ($1, $2, NOW())
    //             ",
    //             chat_uuid,
    //             user.uuid
    //         )
    //         .execute(&self.database_connection)
    //         .await
    //         .map_err(DatabaseError::from_query_error)?;

    //         return Ok(());
    //     }

    //     sqlx::query!(
    //         "
    //         INSERT INTO users_in_chat (chat_uuid, user_uuid, created_at, updated_at)
    //         VALUES ($1, $2, NOW(), NOW())
    //         ",
    //         chat_uuid,
    //         user.uuid
    //     )
    //     .execute(&self.database_connection)
    //     .await
    //     .map_err(DatabaseError::from_query_error)?;

    //     Ok(())
    // }
}
