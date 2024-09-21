use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub uuid: Uuid,
    pub chat_uuid: Uuid,
    pub sender_uuid: Uuid,
    pub file_uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub chat_name: String,
    pub sender_name: String,
    // pub file_duration: f32,
}

impl ChatMessage {
    pub fn from_chat_uuid_sender_uuid_file_uuid(
        chat_uuid: Uuid,
        sender_uuid: Uuid,
        file_uuid: Uuid,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            chat_uuid,
            sender_uuid,
            file_uuid,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            chat_name: "".to_string(),
            sender_name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chat {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub latest_message_created_at: Option<DateTime<Utc>>,
    pub user_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub uuid: Uuid,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn from_name(name: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            user_name: name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}
