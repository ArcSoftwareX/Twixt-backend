use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use self::query::Query;

pub mod handlers;
pub mod query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
