use std::future::ready;

use actix_web::{error::ErrorUnauthorized, http, web::Data, FromRequest, HttpMessage};
use jsonwebtoken::{decode, DecodingKey, Validation};

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

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(app_state.config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(ErrorResponse {
                    message: "Invalid token provided".to_string(),
                })))
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
