use actix_web::{get, post, web::Data, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::model::state::AppState;

#[post("")]
pub async fn graphql_index(data: Data<AppState>, req: GraphQLRequest) -> GraphQLResponse {
    data.schema.execute(req.into_inner()).await.into()
}

#[get("")]
pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
