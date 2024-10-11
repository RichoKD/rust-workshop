use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::course::Course;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistry {
    pub courses: HashMap<u32, Course>,
    pub total_courses: u32,
}
