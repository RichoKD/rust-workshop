use std::collections::HashMap;

use super::{attendance::AttendanceStruct, student::Student};

#[derive(Debug, Clone)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
    pub enrolled_students: Vec<Student>,
    pub attendance_data: HashMap<u32, AttendanceStruct>,
}
