-- Add up migration script here
CREATE TYPE user_user_action AS ENUM ('mute', 'block', 'follow');

CREATE TABLE user_user (
    from_user_id UUID NOT NULL,
    to_user_id UUID NOT NULL,

    CONSTRAINT fk_from_user FOREIGN KEY (from_user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_to_user FOREIGN KEY (to_user_id) REFERENCES users (id) ON DELETE CASCADE,

    action user_user_action NOT NULL,
    
    PRIMARY KEY (from_user_id, to_user_id)
);