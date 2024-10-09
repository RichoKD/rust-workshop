use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};

use crate::types::{course_registry::CourseRegistry, student_registry::StudentRegistry};

use super::{
    course::{create_course, view_course_registry},
    student::register_student,
};

async fn say_gm() -> &'static str {
    "GM"
}

async fn hello_ab() -> &'static str {
    "AB"
}

fn shared_state() -> Arc<Mutex<StudentRegistry>> {
    Arc::new(Mutex::new(StudentRegistry {
        total_students: Vec::new(),
        course_registry: CourseRegistry {
            courses: HashMap::new(),
            total_courses: 0,
        },
    }))
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(say_gm))
        .route("/ab", get(hello_ab))
        .route("/register_student", post(register_student))
        .route("/create_course", post(create_course))
        .route("/course_registry", get(view_course_registry))
        .with_state(shared_state())
}
