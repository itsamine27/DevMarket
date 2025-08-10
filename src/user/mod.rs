use crate::error::{Error, Result};
use crate::{
    State as Mc,
    user::model::{NewUser, User},
};
use axum::response::IntoResponse;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, post, put},
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use tracing::info;
mod model;
#[derive(Deserialize)]
struct LgForm {
    username: String,
    password: String,
}
pub fn user_router() -> Router<Mc> {
    Router::new()
        .route("/register", post(create_user))
        .route("/update/:username", put(update_user))
        .route("/delete", delete(delete_user))
        .route("/login", post(login))
}

async fn create_user(State(mc): State<Mc>, data: Json<NewUser>) -> Result<Json<User>> {
    info!("creating user started");
    let data = mc.create_user(data).await?;
    info!("creating user has been finished");
    Ok(Json(data))
}
async fn update_user(
    State(mc): State<Mc>,
    Path(username): Path<String>,
    data: Json<NewUser>,
) -> Result<Json<User>> {
    info!("updating user started");
    let data = mc.update_user(data, username).await?;
    info!("updating user has been finished");
    Ok(Json(data))
}
async fn delete_user(State(mc): State<Mc>, Path(username): Path<String>) -> Result<Json<User>> {
    info!("deleting user started");
    let data = mc.delete_user(username).await?;
    info!("deleting user finished");
    Ok(Json(data))
}
#[derive(Serialize, Deserialize)]
pub struct Clains {
    username: String,
    exp: usize,
}
impl Clains {
    fn new(username: String) -> Self {
        let now_t = Utc::now()
            .checked_add_signed(Duration::minutes(60))
            .expect("invalide time stamp")
            .timestamp() as usize;
        Self {
            username,
            exp: now_t,
        }
    }
}
#[axum::debug_handler]
async fn login(State(mc): State<Mc>, Json(form): Json<LgForm>) -> Result<impl IntoResponse> {
    let user: User = mc.get_user(form.username.clone()).await?;
    if verify(form.password, &user.password)? {
        let newt = Clains::new(form.username.clone());
        let jwt = encode(
            &Header::default(),
            &newt,
            &EncodingKey::from_secret(mc.JWT_SECRET.as_bytes()),
        )?;
        return Ok(Json(serde_json::json!({ "access_token": jwt })));
    }

    Err(Error::InvalidUser)
}
