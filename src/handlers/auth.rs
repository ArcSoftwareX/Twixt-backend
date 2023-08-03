use actix_web::{
    cookie::{time::Duration as CookieDuration, Cookie},
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::model::{
    auth::{JwtMiddleware, LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    state::AppState,
};

#[post("/signup")]
async fn signup_password(body: Json<RegisterUserSchema>, data: Data<AppState>) -> impl Responder {
    let exists = sqlx::query("SELECT 1 FROM users WHERE username = $1 OR email = $2")
        .bind(body.username.to_owned())
        .bind(body.email.to_owned())
        .fetch_optional(&data.db_pool)
        .await;

    if exists.is_ok() && exists.unwrap().is_some() {
        return HttpResponse::Conflict().json(json!({
            "message": "User already exists"
        }));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (username, name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.username.to_owned(),
        body.name.to_owned(),
        body.email.to_owned(),
        hashed_password
    )
    .fetch_one(&data.db_pool)
    .await;

    match query_result {
        Ok(user) => HttpResponse::Ok().json(json!({ "user": user })),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "message": e.to_string() })),
    }
}

#[post("/login")]
async fn login_password(body: Json<LoginUserSchema>, data: Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_optional(&data.db_pool)
    .await
    .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({ "message": "Invalid username or password" }));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.config.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(CookieDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({ "token": token }))
}

#[post("/logout")]
async fn logout() -> impl Responder {
    let clear_cookie = Cookie::build("token", "")
        .path("/")
        .max_age(CookieDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(clear_cookie)
        .json(json!({ "message": "Logged out successfully" }))
}

#[get("/me")]
async fn get_user(data: Data<AppState>, jwt: JwtMiddleware) -> impl Responder {
    let user_id = jwt.user_id;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1 LIMIT 1",
        user_id.to_owned()
    )
    .fetch_one(&data.db_pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(json!({ "user": user }))
}
