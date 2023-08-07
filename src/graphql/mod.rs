use async_graphql::{EmptySubscription, MergedObject, Schema};

use self::{
    post::{PostsMutation, PostsQuery},
    user::UserQuery,
};

pub mod dataloader;
pub mod handlers;
pub mod post;
pub mod user;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(pub UserQuery, pub PostsQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(pub PostsMutation);
