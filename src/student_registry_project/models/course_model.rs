use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub student_count: u32,
    pub max_capacity: u32,
}
