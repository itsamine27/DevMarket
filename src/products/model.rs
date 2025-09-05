use crate::error::Result;
use crate::{State, error::Error};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, query_as};
#[derive(Debug, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub owner_id: i32,
    pub executable: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: Option<i32>,
    pub rating: Option<i16>,
    pub owner_id: i32,
    pub executable: Option<Vec<u8>>,
}
#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub executable: Option<Vec<u8>>,
}
fn is_exe_file(data: &[u8]) -> bool {
    data.starts_with(&[0x4D, 0x5A])
}

impl State {
    pub async fn new_product(&self, data: Json<NewProduct>) -> Result<Product> {
        if let Some(file) = &data.executable {
            if !is_exe_file(file) {
                return Err(Error::Datatype);
            }
        }

        let store = query_as!(
            Product,
            r#"
            INSERT INTO Product (name, description, price, owner_id, executable)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, price, rating, owner_id, executable
            "#,
            data.name,
            data.description,
            data.price,
            data.owner_id,
            data.executable,
        )
        .fetch_one(&self.pg)
        .await?;

        Ok(store)
    }
    pub async fn all_product(&self, max: i32) -> Result<Vec<Product>> {
        let store = query_as::<_, Product>("SELECT * FROM Product OFFSET $1 LIMIT $2")
            .bind(max)
            .bind(max)
            .fetch_all(&self.pg)
            .await?;
        Ok(store)
    }
    pub async fn delete_product(&self, id: i64) -> Result<Product> {
        let store = query_as!(
            Product,
            "DELETE FROM Product 
            WHERE id = $1
            RETURNING id, name, description, price, rating, owner_id, executable
            ",
            id
        )
        .fetch_one(&self.pg)
        .await?;
        Ok(store)
    }
    pub async fn update_product(&self, id: i64, data: Json<UpdateProduct>) -> Result<Product> {
        if let Some(file) = &data.executable {
            if !is_exe_file(file) {
                return Err(Error::Datatype);
            }
        }
        let store = query_as!(
            Product,
            "UPDATE Product
            SET name=$1, description=$2, price=$3, executable=$4
            WHERE id = $5
            RETURNING id, name, description, price, rating, owner_id, executable",
            data.name,
            data.description,
            data.price,
            data.executable,
            id,
        )
        .fetch_one(&self.pg)
        .await?;
        Ok(store)
    }
    pub async fn get_product(&self, id: i64) -> Result<Product> {
        let store = query_as!(
            Product,
            "SELECT id, name, description, price, rating, owner_id, executable FROM Product
            WHERE id = $1",
            id
        )
        .fetch_one(&self.pg)
        .await?;
        Ok(store)
    }
    pub async fn get_full_product(&self, id: i64) -> Result<Json<Value>> {
        let row: (Value,) = sqlx::query_as(
            r#"
            SELECT to_jsonb(result) FROM (
                SELECT 
                    Product.id, 
                    Product.name, 
                    Product.price, 
                    Product.rating, 
                    "User".username 
                FROM Product
                LEFT JOIN "User" ON Product.owner_id = "User".id
                WHERE Product.id = $1
            ) AS result;
            "#,
        )
        .bind(id)
        .fetch_one(&self.pg)
        .await?;

        Ok(Json(row.0))
    }
}
#[tokio::test]
async fn tt_all() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let sec = std::env::var("jwt_secret").unwrap();
    let state = State {
        pg: pool,
        jwt_secret: sec,
    };
    println!("{:?}", state.all_product(2).await);
    let data: Json<NewProduct> = Json(NewProduct {
        name: "work space".to_string(),
        description: "it is a works space app".to_string(),
        price: 70,
        owner_id: 2,
        executable: std::option::Option::Some(vec![7]),
    });
    let new = state.new_product(data).await.unwrap();
    println!("{new:?}");
    println!("{:?}", state.all_product(2).await);
    let up = Json(UpdateProduct {
        name: "amine".to_string(),
        description: "test description".to_string(),
        price: 77,
        executable: std::option::Option::Some(vec![7]),
    });
    println!("{:?}", state.update_product(new.id, up).await);
    println!("{:?}", state.all_product(2).await);
    println!("{:?}", state.delete_product(new.id).await);
    println!("{:?}", state.all_product(2).await);
}
