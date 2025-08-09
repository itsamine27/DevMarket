use axum::{extract::{Path, State}, routing::{delete, post, put}, Json, Router};
use tracing::info;

use crate::{user::model::{NewUser, User}, State as Mc};
mod model;
pub fn user_router()->Router<Mc> {
    Router::new()
        .route("/register", post(create_user))
        .route("/update/:username", put(update_user))
        .route("/delete", delete(delete_user))
}

async fn create_user(State(mc): State<Mc>, data: Json<NewUser>)-> Json<User>{
    info!("creating user started");
    let data = mc.create_user(data).await;
    info!("creating user has been finished");
    Json(data)
}
async fn update_user(State(mc): State<Mc>, Path(username): Path<String>, data: Json<NewUser>)-> Json<User>{
    info!("updating user started");
    let data = mc.update_user(data, username).await;
    info!("updating user has been finished");
    Json(data)
}
async fn delete_user(State(mc): State<Mc>, Path(username): Path<String>)-> Json<User>{
    info!("deleting user started");
    let data = mc.delete_user(username).await;
    info!("deleting user finished");
    Json(data)
}
