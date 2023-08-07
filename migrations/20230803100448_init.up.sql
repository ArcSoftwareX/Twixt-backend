-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    username VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    photo VARCHAR,
    password VARCHAR(255) NOT NULL,
    
    created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users (username);