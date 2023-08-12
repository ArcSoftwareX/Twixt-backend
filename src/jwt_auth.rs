use std::future::ready;

use actix_web::{error::ErrorUnauthorized, http, web::Data, FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::model::{
    auth::{JwtMiddleware, TokenClaims},
    error::ErrorResponse,
    state::AppState,
};

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let app_state = req.app_data::<Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(ErrorResponse {
                message: "No token provided".to_string(),
            })));
        }

        let user_id = validate_token(token.unwrap(), &app_state.config.jwt_secret);

        if user_id.is_err() {
            return ready(Err(ErrorUnauthorized(ErrorResponse {
                message: "Invalid token provided".to_string(),
            })));
        }

        let user_id = user_id.unwrap();

        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}

pub fn validate_token(token: String, secret: &String) -> anyhow::Result<uuid::Uuid> {
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?
    .claims;

    Ok(uuid::Uuid::parse_str(claims.sub.as_str())?)
}

pub fn generate_token(user_id: String, secret: &String) -> anyhow::Result<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id,
        exp,
        iat,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}
