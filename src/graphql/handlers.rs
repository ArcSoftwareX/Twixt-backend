use actix_web::{get, post, web::Data, HttpRequest, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{jwt_auth::validate_token, model::state::AppState};

#[post("")]
pub async fn graphql_index(
    data: Data<AppState>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = gql_req.into_inner();

    let token = req.cookie("token");

    if let Some(token) = token {
        if let Ok(user_id) = validate_token(token.value().to_string(), &data.config.jwt_secret) {
            gql_req = gql_req.data(user_id);
        }
    }

    data.schema.execute(gql_req).await.into()
}

#[get("")]
pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
