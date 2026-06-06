CREATE TYPE chat_role AS ENUM (
    'user',
    'assistance'
);

CREATE TABLE IF NOT EXISTS chat_messages(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id uuid NOT NULL REFERENCES projects(id),
    role chat_role NOT NULL,
    content text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_chat_message_project_time
ON chat_messages(project_id, created_at DESC);
