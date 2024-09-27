pub mod attendance;
pub mod course;
pub mod course_registry;
pub mod student;
pub mod student_registry;

pub mod basic_types {
    pub use super::attendance;
    pub use super::course;
    pub use super::course_registry;
    pub use super::student;
    pub use super::student_registry;
}
