use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: u32,
    pub name: String,
    pub password: String,
    pub email: String,
}