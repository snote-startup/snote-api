CREATE TYPE task_status AS ENUM (
    'todo',
    'in-progress',
    'done'
);

CREATE TYPE task_priority AS ENUM (
    'low',
    'medium',
    'high'
);

CREATE TABLE IF NOT EXISTS tasks(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id uuid NOT NULL REFERENCES projects(id),
    status task_status NOT NULL DEFAULT 'todo'::task_status,
    priority task_priority NOT NULL,
    content text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
