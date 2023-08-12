use async_graphql::{dataloader::DataLoader, Context, Enum, Object, Result};

use crate::model::{
    auth::{FilteredUser, User, UserId},
    post::{UserToUser, UserToUserAction},
};

use super::dataloader::UsersLoader;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, ctx: &Context<'_>, username: String) -> Result<Option<FilteredUser>> {
        let loader = ctx.data_unchecked::<DataLoader<UsersLoader>>();

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

    async fn user_action(
        &self,
        ctx: &Context<'_>,
        user_id: String,
        action: UserAction,
    ) -> Result<String> {
        let token_user_id = ctx.data::<UserId>()?;

        if token_user_id.to_string() == user_id.trim() {
            return Err("Invalid UserId or Token provided".into());
        }

        let db_pool = &ctx
            .data_unchecked::<DataLoader<UsersLoader>>()
            .loader()
            .db_pool;

        match action {
            UserAction::Follow => {
                let res: UserToUser = sqlx::query_as("INSERT INTO user_user (from_user_id, to_user_id, action) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING *").bind(token_user_id).bind(UserId::parse_str(&user_id)?).bind(UserToUserAction::Follow).fetch_one(db_pool).await?;

                Ok(serde_json::to_string(&res)?)
            }
            UserAction::Unfollow => {
                let res: UserToUser = sqlx::query_as(
                    "DELETE FROM user_user WHERE from_user_id = $1 AND to_user_id = $2 AND action = $3 RETURNING *",
                ).bind(token_user_id).bind(
                    UserId::parse_str(&user_id)?).bind(UserToUserAction::Follow)
                .fetch_one(db_pool)
                .await?;

                Ok(serde_json::to_string(&res)?)
            }
            UserAction::Mute => {
                let res: UserToUser = sqlx::query_as("INSERT INTO user_user (from_user_id, to_user_id, action) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING *").bind(token_user_id).bind(UserId::parse_str(&user_id)?).bind(UserToUserAction::Mute).fetch_one(db_pool).await?;

                Ok(serde_json::to_string(&res)?)
            }
            UserAction::Unmute => {
                let res: UserToUser = sqlx::query_as("DELETE FROM user_user WHERE from_user_id = $1 AND to_user_id = $2 AND action = $3 RETURNING *").bind(token_user_id).bind(UserId::parse_str(&user_id)?).bind(UserToUserAction::Mute).fetch_one(db_pool).await?;

                Ok(serde_json::to_string(&res)?)
            } // not_implemented => Err(format!("{not_implemented:?} is not implemented").into()),
        }
    }
}

#[derive(Eq, PartialEq, Enum, Clone, Copy, Debug)]
pub enum UserAction {
    Follow,
    Unfollow,
    Mute,
    Unmute,
}
