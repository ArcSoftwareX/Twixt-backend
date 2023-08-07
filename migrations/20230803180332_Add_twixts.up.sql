-- Add up migration script here
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,

    author_id UUID NOT NULL,
    CONSTRAINT author FOREIGN KEY (author_id) REFERENCES users (id) ON DELETE CASCADE,

    content VARCHAR (300) NOT NULL,
    image_links VARCHAR (255) array [3],
    video_links VARCHAR (255) array [2],

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW (),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW ()
);

-- CREATE INDEX idx_posts_id ON posts (id)

CREATE TABLE reposts (
    from_id BIGSERIAL NOT NULL,
    to_id BIGSERIAL NOT NULL,

    CONSTRAINT fk_from FOREIGN KEY (from_id) REFERENCES posts (id) ON DELETE CASCADE,
    CONSTRAINT fk_to FOREIGN KEY (to_id) REFERENCES posts (id) ON DELETE CASCADE,

    PRIMARY KEY (from_id, to_id)
);

CREATE TABLE replies (
    user_id UUID NOT NULL,
    post_id BIGSERIAL NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, post_id)
);

CREATE TABLE likes (
    user_id UUID NOT NULL,
    post_id BIGSERIAL NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, post_id)
);