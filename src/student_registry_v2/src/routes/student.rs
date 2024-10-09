use std::collections::HashMap;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::types::student::{Sex, Student};

#[derive(Serialize, Deserialize)]
pub struct StudentRequest {
    first_name: String,
    last_name: String,
    age: u8,
    height: f32,
    sex: char,
}

pub async fn register_student(Json(student): Json<StudentRequest>) -> Json<Student> {
    let sex = match student.sex {
        'M' => Sex::Male,
        'F' => Sex::Female,
        _ => panic!(),
    };
    let student: Student = Student {
        first_name: student.first_name,
        last_name: student.last_name,
        id: 1,
        age: student.age,
        height: student.height,
        sex,
        enrolled_courses: HashMap::new(),
    };
    Json(student)
}
