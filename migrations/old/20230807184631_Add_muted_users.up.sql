-- Add up migration script here
CREATE TABLE muted (
    from_user_id UUID NOT NULL,
    to_user_id UUID NOT NULL,

    CONSTRAINT fk_from_user FOREIGN KEY (from_user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fr_to_user FOREIGN KEY (to_user_id) REFERENCES users (id) ON DELETE CASCADE,

    PRIMARY KEy (from_user_id, to_user_id)
);