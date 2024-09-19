use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Admin {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}