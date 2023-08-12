-- Add up migration script here
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,

    author_id UUID NOT NULL,
    CONSTRAINT author FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE,

    content VARCHAR (300) NOT NULL,
    media_links VARCHAR (255) array [6],

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW ()
);

CREATE TYPE user_post_action AS ENUM ('like', 'reply');
CREATE TYPE post_post_action AS ENUM ('repost');

CREATE TABLE user_post (
    user_id UUID NOT NULL,
    post_id BIGSERIAL NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,

    action user_post_action NOT NULL,

    PRIMARY KEY (user_id, post_id)
);

CREATE TABLE post_post (
    from_id BIGSERIAL NOT NULL,
    to_id BIGSERIAL NOT NULL,

    CONSTRAINT fk_from FOREIGN KEY (from_id) REFERENCES posts (id) ON DELETE CASCADE,
    CONSTRAINT fk_to FOREIGN KEY (to_id) REFERENCES posts (id) ON DELETE CASCADE,

    action post_post_action NOT NULL,

    PRIMARY KEY (from_id, to_id)
);