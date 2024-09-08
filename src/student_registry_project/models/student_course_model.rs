use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentCourse {
    pub student_id: u32,  // Foreign key to students table
    pub course_id: u32,   // Foreign key to courses table
    pub grade: Option<u8>,
    pub session_id: u32, // Foreign key to sessions table
}
