use std::collections::HashMap;

use uuid::Uuid;

use crate::{domain::models::User, error::DatabaseError, DatabaseConnection, Result};

pub struct UsersRepo {
    database_connection: DatabaseConnection,
}

impl UsersRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    async fn find_all_users(&self) -> Result<HashMap<Uuid, User>> {
        let rows = sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().map(|user| (user.uuid, user)).collect())
    }

    async fn find_user_by_uuid(&self, uuid: Uuid) -> Result<Option<User>> {
        let row = sqlx::query_as!(
            User,
            "
            SELECT *
            FROM users
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row)
    }

    async fn commit_user(&self, user: User) -> Result<Option<User>> {
        if let Some(_) = self.find_user_by_uuid(user.uuid).await? {
            sqlx::query!(
                "
                UPDATE users
                SET user_name = $2, updated_at = NOW(), deleted_at = $3
                WHERE uuid = $1
                ",
                user.uuid,
                user.user_name,
                user.deleted_at
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return self.find_user_by_uuid(user.uuid).await;
        }

        sqlx::query_as!(
            User,
            "
            INSERT INTO users (uuid, user_name, created_at, updated_at, deleted_at)
            VALUES ($1, $2, NOW(), NOW(), $3)
            ",
            user.uuid,
            user.user_name,
            user.deleted_at
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        self.find_user_by_uuid(user.uuid).await
    }
}
