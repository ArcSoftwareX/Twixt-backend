-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE twixts (
    id SERIAL PRIMARY KEY,

    author_id UUID,
    CONSTRAINT author FOREIGN KEY (author_id) REFERENCES users (id),

    content VARCHAR (300) NOT NULL,
    image_links VARCHAR (255) array [3],
    video_links VARCHAR (255) array [2]
);

CREATE TABLE retwixts (
    from_id INT NOT NULL,
    to_id INT NOT NULL,

    CONSTRAINT fk_from FOREIGN KEY (from_id) REFERENCES twixts (id) ON DELETE CASCADE,
    CONSTRAINT fk_to FOREIGN KEY (to_id) REFERENCES twixts (id) ON DELETE CASCADE,

    PRIMARY KEY (from_id, to_id)
);

CREATE TABLE likes (
    user_id UUID NOT NULL,
    twixt_id INT NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_twixt FOREIGN KEY (twixt_id) REFERENCES twixts (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, twixt_id)
);