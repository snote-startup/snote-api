CREATE TYPE conversation_role AS ENUM (
    'user',
    'assistance'
);

CREATE TABLE IF NOT EXISTS llm_conversation_histories(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id uuid NOT NULL REFERENCES projects(id),
    role conversation_role NOT NULL,
    content text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
