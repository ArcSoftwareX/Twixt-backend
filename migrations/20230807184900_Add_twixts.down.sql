-- Add down migration script here
DROP TABLE posts;
DROP TABLE user_post;
DROP TABLE post_post;
DROP TYPE user_post_action;
DROp TYPE post_post_action;