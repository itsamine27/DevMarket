use axum::{
    extract::{Path, State}, routing::{delete, get, post, put}, Json, Router
};
use tracing::info;

use crate::{products::model::{NewProduct, Product, UpdateProduct}, State as Mc};
mod model;
pub fn product_route() -> Router<Mc> {
    Router::new()
        .route("/", post(new_product))
        .route("/", get(all_product))
        .route("/:id", delete(delete_product))
        .route("/:id", put(update_product))
}
async fn new_product(State(mc): State<Mc>, data: Json<NewProduct>) -> Json<Product> {
    info!("starting new product");
    let pool = mc.new_product(data).await;
    info!("new product inserted");
    Json(pool)
}
async fn all_product(State(mc): State<Mc>) -> Json<Vec<Product>> {
    info!("starting to fetch all products");
    let data = mc.all_product().await;
    info!("all data has been fetched");
    Json(data)
}
async fn delete_product(State(mc): State<Mc>, Path(id): Path<i64>) -> Json<Product> {
    info!("starting to delete product");
    let data = mc.delete_product(id).await;
    info!("product has been deleted");
    Json(data)
}
#[axum::debug_handler]
async fn update_product(State(mc): State<Mc>, Path(id): Path<i64>, data: Json<UpdateProduct>)-> Json<Product>{
    info!("updating started");
    let pool = mc.update_product(id, data).await;
    info!("finished updating product");
    Json(pool)
}
