use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};

use crate::graphql::dataloader::{HasLikeInput, PostsLoader, UsersLoader};

use super::auth::{FilteredUser, UserId};

pub type PostId = i64;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Post {
    pub id: PostId,

    pub author_id: UserId,

    pub content: String,
    pub image_links: Option<Vec<String>>,
    pub video_links: Option<Vec<String>>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct FilteredPost {
    pub id: PostId,

    pub author_id: String,

    #[graphql(skip)]
    pub author_uuid: UserId,

    pub content: String,
    pub image_links: Option<Vec<String>>,
    pub video_links: Option<Vec<String>>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[ComplexObject]
impl FilteredPost {
    async fn author(&self, ctx: &Context<'_>) -> Result<FilteredUser> {
        let users_loader = ctx.data_unchecked::<DataLoader<UsersLoader>>();

        let user = users_loader.load_one(self.author_uuid).await?.unwrap();

        Ok(FilteredUser {
            id: user.id.to_string(),
            username: user.username,
            name: user.name,
            email: user.email,
            photo: user.photo,
            created_at: user.created_at.map(|val| val.to_string()),
            updated_at: user.updated_at.map(|val| val.to_string()),
        })
    }

    async fn is_liked_by(&self, ctx: &Context<'_>, user_id: String) -> Result<bool> {
        let posts_loader = ctx.data_unchecked::<DataLoader<PostsLoader>>();

        let res = posts_loader
            .load_one(HasLikeInput {
                post_id: self.id,
                user_id: UserId::parse_str(&user_id)?,
            })
            .await?;

        Ok(res.is_some() && res.unwrap() == self.id)
    }
}
