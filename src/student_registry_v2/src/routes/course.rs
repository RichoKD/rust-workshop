use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::types::{
    course::Course, course_registry::CourseRegistry, student_registry::StudentRegistry,
};

#[derive(Serialize, Deserialize)]
pub struct CreateCourse {
    pub name: String,
    pub capacity: u32,
}

pub async fn create_course(
    State(state): State<Arc<Mutex<StudentRegistry>>>,
    Json(payload): Json<CreateCourse>,
) -> Json<Course> {
    let mut data = state.lock().expect("mutex was poisoned");
    let id = data.course_registry.courses.len() as u32 + 1;
    let course: Course = Course {
        id,
        attendance_data: HashMap::new(),
        enrolled_students: Vec::with_capacity(payload.capacity as usize),
        capacity: payload.capacity,
        name: payload.name,
    };
    data.course_registry.courses.insert(id, course.clone());
    data.course_registry.total_courses += 1;
    Json(course)
}

pub async fn view_course_registry(
    State(state): State<Arc<Mutex<StudentRegistry>>>,
) -> Json<CourseRegistry> {
    Json(state.lock().unwrap().course_registry.clone())
}
