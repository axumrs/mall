use std::sync::Arc;

use axum::Extension;

use crate::{pb, types::state::user::State, utils::dt};

pub async fn register(Extension(state): Extension<Arc<State>>) -> String {
    let mut client = state.client.clone();
    let dateline = dt::chrono2prost(&chrono::Local::now());
    let resp = client
        .create_user(pb::User {
            email: "team@axum.rs".to_string(),
            nickname: "root".to_string(),
            password: "foobar".to_string(),
            dateline,
            ..Default::default()
        })
        .await
        .unwrap();
    resp.into_inner().value.to_string()
}
