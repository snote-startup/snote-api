CREATE TABLE IF NOT EXISTS quotas(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id uuid UNIQUE NOT NULL REFERENCES accounts(id),
    count int NOT NULL DEFAULT 5
);
