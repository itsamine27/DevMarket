use crate::State;
use crate::error::Result;
use axum::Json;
use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "Roles", rename_all = "lowercase")]
pub enum Role {
    Admin,
    Seller,
    Buyer,
}

#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
    role: Role,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct NewUser {
    email: String,
    username: String,
    password: String,
    role: Role,
}
impl State {
    pub async fn create_user(&self, data: Json<NewUser>) -> Result<User> {
        let password = hash(data.password.clone(), DEFAULT_COST).unwrap();
        let quer = query_as::<_, User>(
            r#"
            INSERT INTO "User" (email, username, password, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password, role
            "#,
        )
        .bind(&data.email)
        .bind(&data.username)
        .bind(&password)
        .bind(&data.role)
        .fetch_one(&self.pg)
        .await?;
        Ok(quer)
    }
    pub async fn update_user(&self, data: Json<NewUser>, username: String) -> Result<User> {
        let quer = query_as::<_, User>(
            r#"
            UPDATE "User",
            SET username = $1, email =$2, password = $3, role=$4
            ON username = $5
            "#,
        )
        .bind(data.username.clone())
        .bind(data.email.clone())
        .bind(data.password.clone())
        .bind(data.role.clone())
        .bind(username)
        .fetch_one(&self.pg)
        .await?;
        Ok(quer)
    }
    pub async fn delete_user(&self, username: String) -> Result<User> {
        let quer = query_as::<_, User>(
            r#"
            DELETE FROM "User"
            WHERE username = $1
            RETURNING id, email, username, password, role
            "#,
        )
        .bind(username)
        .fetch_one(&self.pg)
        .await?;
        Ok(quer)
    }
    pub async fn all_user(&self) -> Result<Vec<User>> {
        Ok(query_as::<_, User>(
            r#"
            SELECT * FROM "User"
            "#,
        )
        .fetch_all(&self.pg)
        .await?)
    }
}

#[tokio::test]
async fn user_t() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let state = State { pg: pool };
    println!("{:?}", state.all_user().await);
    let data = Json(NewUser {
        email: "amine@gmail.com".to_string(),
        username: "aminou".to_string(),
        password: "6;;356gd".to_string(),
        role: Role::Seller,
    });
    let new = state.create_user(data).await.unwrap();
    println!("{:?}", state.all_user().await);
    println!("{:?}", state.delete_user(new.username).await);
    println!("{:?}", state.all_user().await);
}
