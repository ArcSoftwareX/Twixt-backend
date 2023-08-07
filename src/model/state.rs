use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
use sqlx::{Pool, Postgres};

use crate::{
    config::Config,
    graphql::{
        dataloader::{PostsLoader, UsersLoader},
        AppSchema, Mutation, Query,
    },
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
        let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(DataLoader::new(
                PostsLoader {
                    db_pool: db_pool.clone(),
                },
                tokio::spawn,
            ))
            .data(DataLoader::new(
                UsersLoader {
                    db_pool: db_pool.clone(),
                },
                tokio::spawn,
            ))
            .data(config.clone())
            .finish();

        Ok(Self {
            config,
            db_pool,
            schema,
        })
    }
}
