use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub id: u32,
    pub age: u8,
    pub height: f32,
    pub sex: Sex,
    pub enrolled_courses: HashMap<u32, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
}
