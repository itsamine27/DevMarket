use crate::error::Result;
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
}
async fn new_product(State(mc): State<Mc>, data: Json<NewProduct>) -> Result<Json<Product>> {
    info!("starting new product");
    let data = mc.new_product(data).await?;
    info!("new product inserted");
    Ok(Json(data))
}
#[axum::debug_handler]
async fn all_product(State(mc): State<Mc>, Query(meta): Query<Qer>) -> Result<Json<Vec<Product>>> {
    let max = meta.page.unwrap_or(1) * 10;
    info!("starting to fetch all products");
    let data = mc.all_product(max).await?;
    info!("all data has been fetched");
    Ok(Json(data))
}
async fn delete_product(State(mc): State<Mc>, Path(id): Path<i64>) -> Result<Json<Product>> {
    info!("starting to delete product");
    let data = mc.delete_product(id).await?;
    info!("product has been deleted");
    Ok(Json(data))
}
#[axum::debug_handler]
async fn update_product(
    State(mc): State<Mc>,
    Path(id): Path<i64>,
    data: Json<UpdateProduct>,
) -> Result<Json<Product>> {
    info!("updating started");
    let pool = mc.update_product(id, data).await?;
    info!("finished updating product");
    Ok(Json(pool))
}
