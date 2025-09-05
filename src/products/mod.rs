use crate::error::{Error, Result};
use crate::ext::IsAuth;
use crate::user::model::Role;
use crate::{
    State as Mc,
    products::model::{NewProduct, Product, UpdateProduct},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;
#[derive(Deserialize)]
struct Qer {
    page: Option<i32>,
}
mod model;
pub fn product_route() -> Router<Mc> {
    Router::new()
        .route("/", post(new_product))
        .route("/", get(all_product))
        .route("/:id", delete(delete_product))
        .route("/:id", put(update_product))
        .route("/:id", get(get_product))
}
async fn new_product(
    IsAuth(ext): IsAuth,
    State(mc): State<Mc>,
    data: Json<NewProduct>,
) -> Result<Json<Product>> {
    if (ext.role == Role::Seller) || (ext.role == Role::Admin) {
        info!("starting new product");
        let data = mc.new_product(data).await?;
        info!("new product inserted");
        return Ok(Json(data));
    }
    Err(Error::InvalidUser)
}
#[axum::debug_handler]
async fn all_product(State(mc): State<Mc>, Query(meta): Query<Qer>) -> Result<Json<Vec<Product>>> {
    let max = meta.page.unwrap_or(1) * 10;
    info!("starting to fetch all products");
    let data = mc.all_product(max).await?;
    info!("all data has been fetched");
    Ok(Json(data))
}
async fn delete_product(
    IsAuth(ext): IsAuth,
    State(mc): State<Mc>,
    Path(id): Path<i64>,
) -> Result<Json<Product>> {
    let product = mc.get_product(id).await?;
    let owner = mc.get_user(ext.username).await?;
    if product.owner_id == owner.id || ext.role == Role::Admin {
        info!("starting to delete product");
        let data = mc.delete_product(id).await?;
        info!("product has been deleted");
        return Ok(Json(data));
    }
    Err(Error::InvalidUser)
}
#[axum::debug_handler]
async fn update_product(
    IsAuth(ext): IsAuth,
    State(mc): State<Mc>,
    Path(id): Path<i64>,
    data: Json<UpdateProduct>,
) -> Result<Json<Product>> {
    let product = mc.get_product(id).await?;
    let owner = mc.get_user(ext.username).await?;
    if product.owner_id == owner.id || ext.role == Role::Admin {
        info!("updating started");
        let pool = mc.update_product(id, data).await?;
        info!("finished updating product");
        return Ok(Json(pool));
    }
    Err(Error::InvalidUser)
}
async fn get_product(State(mc): State<Mc>, Path(id): Path<i64>) -> Result<Json<Value>> {
    info!("fetching product started");
    let pool = mc.get_full_product(id).await?;
    info!("fetching product went ");
    Ok(pool)
}
