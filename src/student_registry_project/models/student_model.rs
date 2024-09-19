use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};
// use sqlx::{FromRow, Decode};

// #[derive(Debug, Clone, Serialize, Deserialize, sqlx::{FromRow, Decode})]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i8,
    pub height: f32,
    pub sex: Sex,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum Sex {
    Male,
    Female,
}
