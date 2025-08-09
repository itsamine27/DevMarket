use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as};
use bcrypt::{hash, DEFAULT_COST};
use crate::State;
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
    role: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct NewUser {
    email: String,
    username: String,
    password: String,
    role: String,
}
impl State {
    pub async fn create_user(&self, data: Json<NewUser>) -> User {
        let password = hash(data.password.clone(), DEFAULT_COST).unwrap();
        query_as::<_, User>(
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
        .await
        .expect("an error has occured when creating a user")
    }
    pub async fn update_user(&self, data: Json<NewUser>, username:String) -> User {
        query_as::<_, User>(
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
        .await
        .expect("a problem has occured when creating teh user")
    }
    pub async fn delete_user(&self, username:String) -> User {
        query_as::<_, User>(
            r#"
            DELETE FROM "User"
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&self.pg)
        .await
        .expect("a problem has occured when deleting user ")
    }
    pub async fn all_user(&self)-> Vec<User>{
        query_as::<_,User>(
            r#"
            SELECT * FROM "User"
            "#
        ).fetch_all(&self.pg)
        .await
        .expect("a proble has occured when trying to fetch all data")
    }
}

