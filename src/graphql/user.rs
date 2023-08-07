use async_graphql::{dataloader::DataLoader, Context, Object, Result};

use crate::model::auth::{FilteredUser, User};

use super::dataloader::PostsLoader;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, ctx: &Context<'_>, username: String) -> Result<Option<FilteredUser>> {
        let loader = ctx.data_unchecked::<DataLoader<PostsLoader>>();

        let user: Option<User> = loader.load_one(username).await.unwrap();

        let user = user.map(|user| FilteredUser {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
            username: user.username,
            photo: user.photo,
            created_at: user.created_at.map(|val| val.to_string()),
            updated_at: user.updated_at.map(|val| val.to_string()),
        });

        Ok(user)
    }
}
