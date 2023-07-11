use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use mall::{pb, web_api};

#[tokio::main]
async fn main() {
    let cli = pb::user_service_client::UserServiceClient::connect("http://127.0.0.1:9527")
        .await
        .unwrap();
    let state = mall::types::state::user::State { client: cli };

    let app: Router = Router::new()
        .route("/register", get(web_api::user_api::register))
        .layer(Extension(Arc::new(state)));

    axum::Server::bind(&"127.0.0.1:9528".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
