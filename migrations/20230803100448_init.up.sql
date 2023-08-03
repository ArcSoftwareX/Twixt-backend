-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    id UUID PRIMARY KEY DEFAULT (uuid_generate_v4()),
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