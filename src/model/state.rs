use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::{Pool, Postgres};

use crate::{
    config::Config,
    graphql::{query::Query, AppSchema},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db_pool: Pool<Postgres>,
    pub schema: AppSchema,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db_pool = sqlx::postgres::PgPool::connect(&config.db_url).await?;
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(db_pool.clone())
            .finish();

        Ok(Self {
            config,
            db_pool,
            schema,
        })
    }
}
