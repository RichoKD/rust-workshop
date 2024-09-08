use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub height: f32,
    pub sex: Sex,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}
