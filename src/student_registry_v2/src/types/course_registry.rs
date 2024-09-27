use std::collections::HashMap;

use super::course::Course;

#[derive(Debug, Clone)]
pub struct CourseRegistry {
    pub courses: HashMap<u32, Course>,
    pub total_courses: u32,
}