use crate::error::{Error, Result};
use crate::ext::IsAuth;
use crate::user::model::Role;
use crate::{
    State as Mc,
    user::model::{NewUser, User},
};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, post, put},
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info};
pub mod model;
#[derive(Deserialize, Serialize)]
struct LgForm {
    username: String,
    password: String,
}
#[derive(Deserialize, Serialize)]
struct NewUserResp {
    user: User,
    message: Value,
}

pub fn user_router() -> Router<Mc> {
    Router::new()
        .route("/register", post(create_user))
        .route("/update/:username", put(update_user))
        .route("/delete", delete(delete_user))
        .route("/login", post(login))
        .route("/:username", get(get_user))
}

async fn create_user(State(mc): State<Mc>, data: Json<NewUser>) -> Result<impl IntoResponse> {
    info!("creating user started");
    let data = mc.create_user(data).await?;
    info!("creating user has been finished");
    info!("login started");
    let login_data = Json(LgForm {
        username: data.username.clone(),
        password: data.password.clone(),
    });
    let Json(value) = login(State(mc), login_data).await?;
    let login_info: LgForm = serde_json::from_value(value)?;

    let response = NewUserResp {
        user: data,
        message: serde_json::to_value(login_info)?,
    };
    info!("login went successfuly");
    Ok(Json(response))
}
async fn update_user(
    IsAuth(ext): IsAuth,
    State(mc): State<Mc>,
    Path(username): Path<String>,
    data: Json<NewUser>,
) -> Result<Json<User>> {
    if ext.username == username || ext.role == Role::Admin {
        info!("updating user started");
        let data = mc.update_user(data, username).await?;
        info!("updating user has been finished");
        return Ok(Json(data));
    }
    Err(Error::InvalidUser)
}
async fn delete_user(
    IsAuth(ext): IsAuth,
    State(mc): State<Mc>,
    Path(username): Path<String>,
) -> Result<Json<User>> {
    if username == ext.username || ext.role == Role::Admin {
        info!("deleting user started");
        let data = mc.delete_user(username).await?;
        info!("deleting user finished");
        return Ok(Json(data));
    }
    Err(Error::InvalidUser)
}
async fn get_user(State(mc): State<Mc>, Path(username): Path<String>) -> Result<Json<User>> {
    info!("fetching user started");
    let data = mc.get_user(username).await?;
    info!("fetching user finished");
    Ok(Json(data))
}
#[derive(Serialize, Deserialize)]
pub struct Clains {
    pub username: String,
    pub role: Role,
    pub exp: usize,
}
impl Clains {
    fn new(username: String, role: Role) -> Result<Self> {
        let now_t = usize::try_from(
            Utc::now()
                .checked_add_signed(Duration::minutes(60))
                .expect("invalide time stamp")
                .timestamp(),
        );
        let data = Self {
            username,
            role,
            exp: now_t?,
        };
        Ok(data)
    }
    pub fn from_token(token: &str) -> Result<Self> {
        let secret = std::env::var("JWT_SECRET")?;

        let validation = Validation::new(Algorithm::HS256);

        let token_data: TokenData<Self> = decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;

        // Return the claims
        Ok(token_data.claims)
    }
}
#[axum::debug_handler]
async fn login(State(mc): State<Mc>, Json(form): Json<LgForm>) -> Result<Json<Value>> {
    info!("starting user login");
    let user: User = mc.get_user(form.username.clone()).await?;
    info!("username has been fetched");
    if verify(form.password, &user.password)? {
        let newt = Clains::new(form.username.clone(), user.role)?;
        let jwt = encode(
            &Header::default(),
            &newt,
            &EncodingKey::from_secret(mc.jwt_secret.as_bytes()),
        )?;
        info!("ur successfuly loged in");
        return Ok(Json(serde_json::json!({ "access_token": jwt })));
    }
    debug!("sthg went wrong when loging in");
    Err(Error::InvalidUser)
}
