use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u32, // Is year
    pub name: String,
}
