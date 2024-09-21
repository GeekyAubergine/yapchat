-- Add up migration script here
CREATE TABLE chat_messages (
    uuid UUID NOT NULL,
    chat_uuid UUID NOT NULL,
    sender_uuid UUID NOT NULL,
    file_uuid UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
)
