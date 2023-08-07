use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::Loader;
use sqlx::{Pool, Postgres};

use crate::model::{
    auth::{User, UserId},
    post::{FilteredPost, Post, PostId},
};

use super::post::Like;

#[derive(Debug)]
pub struct PostsLoader {
    pub db_pool: Pool<Postgres>,
}

// Load by username

#[async_trait::async_trait]
impl Loader<String> for PostsLoader {
    type Value = User;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("executing load by username: {keys:?}");

        let res: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE username = ANY ($1)")
            .bind(keys)
            .fetch_all(&self.db_pool)
            .await?;
        let res = res.iter().cloned().fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.username.clone(), x);
            acc
        });

        Ok(res)
    }
}

#[async_trait::async_trait]
impl Loader<PostId> for PostsLoader {
    type Value = FilteredPost;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        println!("executing load by post id: {keys:?}");

        let res: Vec<Post> = sqlx::query_as("SELECT * FROM posts WHERE id = ANY ($1)")
            .bind(keys)
            .fetch_all(&self.db_pool)
            .await?;
        let res = res.iter().cloned().fold(HashMap::new(), |mut acc, x| {
            acc.insert(
                x.id,
                FilteredPost {
                    id: x.id,
                    author_id: x.author_id.to_string(),
                    author_uuid: x.author_id,
                    content: x.content,
                    image_links: x.image_links,
                    video_links: x.video_links,
                    created_at: x.created_at.map(|val| val.to_string()),
                    updated_at: x.updated_at.map(|val| val.to_string()),
                },
            );
            acc
        });

        Ok(res)
    }
}

// Used for likes retrieval

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct HasLikeInput {
    pub user_id: UserId,
    pub post_id: PostId,
}

#[async_trait::async_trait]
impl Loader<HasLikeInput> for PostsLoader {
    type Value = i64;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[HasLikeInput],
    ) -> Result<HashMap<HasLikeInput, Self::Value>, Self::Error> {
        let res = sqlx::query_as!(
            Like,
            "SELECT * FROM likes WHERE user_id = ANY ($1) AND post_id = ANY ($2)",
            &keys
                .iter()
                .map(|val| val.user_id.to_owned())
                .collect::<Vec<UserId>>(),
            &keys.iter().map(|val| val.post_id).collect::<Vec<PostId>>()
        )
        .fetch_all(&self.db_pool)
        .await?;

        println!("before hashmapping: {res:#?}");

        let res = res.iter().fold(HashMap::new(), |mut acc, x| {
            acc.insert(
                HasLikeInput {
                    user_id: x.user_id,
                    post_id: x.post_id,
                },
                x.post_id,
            );
            acc
        });

        println!("likes: {res:#?}");

        Ok(res)
    }
}

pub struct UsersLoader {
    pub db_pool: Pool<Postgres>,
}

// Load by user id

#[async_trait::async_trait]
impl Loader<UserId> for UsersLoader {
    type Value = User;
    type Error = Arc<sqlx::Error>;

    async fn load(
        &self,
        keys: &[UserId],
    ) -> std::result::Result<HashMap<UserId, Self::Value>, Self::Error> {
        println!("get users for: {:#?}", keys);

        let res: Vec<User> = sqlx::query_as("SELECT * FROM users WHERE id = ANY ($1)")
            .bind(keys)
            .fetch_all(&self.db_pool)
            .await?;

        let res = res.iter().cloned().fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.id, x);
            acc
        });

        Ok(res)
    }
}
