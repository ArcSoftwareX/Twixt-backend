use async_graphql::SimpleObject;
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// User

pub type UserId = uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub name: String,
    pub email: String,
    pub photo: Option<String>,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Debug, Clone)]
pub struct FilteredUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub photo: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

// JWT

pub struct JwtMiddleware {
    pub user_id: UserId,
}
