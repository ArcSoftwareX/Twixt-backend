use async_graphql::{dataloader::DataLoader, Context, Object, Result};

use crate::{
    config::Config,
    jwt_auth::validate_token,
    model::{
        auth::UserId,
        post::{FilteredPost, Post, PostAction, PostId, PostInput, UserToPost, UserToPostAction},
    },
};

use super::dataloader::PostsLoader;

#[derive(Default)]
pub struct PostsQuery;

#[Object]
impl PostsQuery {
    async fn get_post(&self, ctx: &Context<'_>, id: PostId) -> Result<Option<FilteredPost>> {
        let loader = ctx.data_unchecked::<DataLoader<PostsLoader>>();

        let post = loader.load_one(id).await?;

        Ok(post)
    }

    async fn get_posts(&self, ctx: &Context<'_>, page: u32) -> Result<Vec<FilteredPost>> {
        let db_pool = &ctx
            .data_unchecked::<DataLoader<PostsLoader>>()
            .loader()
            .db_pool;

        let res: Vec<Post> = sqlx::query_as("SELECT * FROM posts OFFSET $1 LIMIT 20")
            .bind((page * 20) as i64)
            .fetch_all(db_pool)
            .await?;

        let res = res
            .iter()
            .map(|val| FilteredPost {
                id: val.id,
                author_id: val.author_id.to_string(),
                author_uuid: val.author_id,
                content: val.content.to_owned(),
                created_at: val.created_at.map(|val| val.to_string()),
                updated_at: val.updated_at.map(|val| val.to_string()),
                image_links: val.image_links.clone(),
                video_links: val.video_links.clone(),
            })
            .collect::<Vec<_>>();

        Ok(res)
    }
}

#[derive(Default)]
pub struct PostsMutation;

#[Object]
impl PostsMutation {
    async fn create_post(&self, ctx: &Context<'_>, post: PostInput) -> Result<FilteredPost> {
        let user_id = ctx.data::<UserId>()?;

        let db_pool = &ctx
            .data_unchecked::<DataLoader<PostsLoader>>()
            .loader()
            .db_pool;

        let res = sqlx::query_as!(Post, "INSERT INTO posts (author_id, content, image_links, video_links) VALUES ($1, $2, $3, $4) RETURNING *", user_id, post.content, post.image_links.as_deref(), post.video_links.as_deref()).fetch_one(db_pool).await?;

        Ok(FilteredPost {
            id: res.id,
            content: res.content,
            author_id: res.author_id.to_string(),
            author_uuid: res.author_id,
            created_at: res.created_at.map(|val| val.to_string()),
            updated_at: res.updated_at.map(|val| val.to_string()),
            image_links: res.image_links,
            video_links: res.video_links,
        })
    }

    async fn post_action(
        &self,
        ctx: &Context<'_>,
        token: String,
        post_id: u32,
        action: PostAction,
    ) -> Result<String> {
        let jwt_secret = &ctx.data_unchecked::<Config>().jwt_secret;

        let user_id = validate_token(token, jwt_secret)?;

        let db_pool = &ctx
            .data_unchecked::<DataLoader<PostsLoader>>()
            .loader()
            .db_pool;

        match action {
            PostAction::Like => {
                let res: UserToPost = sqlx::query_as(
                    r#"INSERT INTO user_post (user_id, post_id, action) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING *"#,
                ).bind(user_id).bind(post_id as PostId).bind(UserToPostAction::Like as UserToPostAction)
                .fetch_one(db_pool)
                .await?;

                Ok(serde_json::to_string(&res)?)
            }
            PostAction::RemoveLike => {
                let res: UserToPost = sqlx::query_as(
                    "DELETE FROM likes WHERE user_id = $1 AND post_id = $2 AND action = $3 RETURNING *"
                ).bind(user_id).bind(post_id as PostId)
                .fetch_one(db_pool)
                .await?;

                Ok(serde_json::to_string(&res)?)
            } // not_implemented => Err(format!("{:?} is not implemented", not_implemented).into()),
        }
    }
}
