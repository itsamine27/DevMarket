use crate::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, query_as};

#[derive(Debug, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub owner_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: Option<i32>,
    pub rating: Option<i16>,
    pub owner_id: i32,
}
#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
}
impl State {
    pub async fn new_product(&self, data: Json<NewProduct>) -> Product {
        let store = query_as!(
            Product,
            r#"
            INSERT INTO Product (name, description, price, owner_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, price, rating, owner_id
            "#,
            data.name,
            data.description,
            data.price,
            data.owner_id
        )
        .fetch_one(&self.pg)
        .await
        .expect("Failed to insert product");

        store
    }
    pub async fn all_product(&self, max:i32) -> Vec<Product> {
        let store = query_as::<_,Product>("SELECT * FROM Product OFFSET $1 LIMIT $2")
            .bind(max)
            .bind(max)
            .fetch_all(&self.pg)
            .await
            .expect("failed to get data");
        store
    }
    pub async fn delete_product(&self, id: i64) -> Product {
        let store = query_as!(
            Product,
            "DELETE FROM Product 
            WHERE id = $1
            RETURNING id, name, description, price, rating, owner_id
            ",
            id
        )
        .fetch_one(&self.pg)
        .await
        .expect("a problem has occured in the delete");
        store
    }
    pub async fn update_product(&self, id: i64, data: Json<UpdateProduct>) -> Product {
        let store = query_as!(
            Product,
            "UPDATE Product
            SET name=$1, description=$2, price=$3
            WHERE id = $4
            RETURNING id, name, description, price, rating, owner_id",
            data.name,
            data.description,
            data.price,
            id,
        )
        .fetch_one(&self.pg)
        .await
        .expect("a problem has occured when updating data");
        store
    }
}
#[tokio::test]
async fn tt_all() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("invalid url path");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let state = State { pg: pool };
    println!("{:?}", state.all_product(2).await);
    let data: Json<NewProduct> = Json(NewProduct {
        name: "work space".to_string(),
        description: "it is a works space app".to_string(),
        price: 70,
        owner_id: 2,
    });
    let new = state.new_product(data).await;
    println!("{new:?}");
    println!("{:?}", state.all_product(2).await);
    let up = Json(UpdateProduct {
        name: "amine".to_string(),
        description: "test description".to_string(),
        price: 77,
    });
    println!("{:?}", state.update_product(new.id, up).await);
    println!("{:?}", state.all_product(2).await);
    println!("{:?}", state.delete_product(new.id).await);
    println!("{:?}", state.all_product(2).await);
}
