#![warn(warnings)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use axum::{
    Router,
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    serve,
};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
#[cfg(test)]
mod test;
use crate::error::Result;
use crate::{products::product_route, user::user_router};
mod error;
mod ext;
mod products;
mod user;
#[derive(Clone)]
struct State {
    pg: PgPool,
    jwt_secret: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    let secret = std::env::var("jwt_secret")?;
    let state = State {
        pg: pool,
        jwt_secret: secret,
    };
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/:name", get(hello))
        .nest("/products", product_route())
        .nest("/auth", user_router())
        .with_state(state);

    let sock = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("lisening on port {sock}");
    let app = TcpListener::bind(sock).await?;
    serve(app, router).await?;
    Ok(())
}
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("hello {name}"))
}
