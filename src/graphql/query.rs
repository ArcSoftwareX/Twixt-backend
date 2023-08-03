use async_graphql::{Context, Object, Result};
use sqlx::{Pool, Postgres};

use crate::model::auth::{FilteredUser, User};

pub struct Query;

#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>, username: String) -> Result<Option<FilteredUser>> {
        let db = ctx.data::<Pool<Postgres>>()?;
        let res = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(db)
            .await?
            .map(|user| FilteredUser {
                id: user.id.to_string(),
                username: user.username,
                name: user.name,
                email: user.email,
                photo: user.photo,
                created_at: user.created_at.map(|val| val.to_string()),
                updated_at: user.updated_at.map(|val| val.to_string()),
            });

        Ok(res)
    }
}
