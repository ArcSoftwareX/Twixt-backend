use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use config::Config;
use graphql::handlers::{graphql_index, graphql_playground};
use handlers::{
    auth::{get_user, login_password, logout, signup_password},
    file::file_handler,
    health::health,
    posts::create_post,
};
use model::state::AppState;

mod config;
mod graphql;
mod handlers;
mod jwt_auth;
mod model;
mod prelude;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    utils::log_init();
    utils::storage_init()?;

    let config = Config::init().unwrap();
    let app_state = AppState::new(config).await.unwrap();
    let app_state = Data::new(app_state);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .allowed_methods(["GET", "POST"])
            .supports_credentials();

        App::new()
            .service(
                web::scope("/api")
                    .service(health)
                    .service(
                        web::scope("/auth")
                            .service(login_password)
                            .service(signup_password)
                            .service(get_user)
                            .service(logout),
                    )
                    .service(file_handler)
                    .service(web::scope("/post").service(create_post)),
            )
            .service(
                web::scope("/graphql")
                    .service(graphql_index)
                    .service(graphql_playground),
            )
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
