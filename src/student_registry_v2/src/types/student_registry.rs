use super::{course_registry::CourseRegistry, student::Student};

#[derive(Debug, Clone)]
pub struct StudentRegistry {
    pub total_students: Vec<Student>,
    pub course_registry: CourseRegistry,
}
