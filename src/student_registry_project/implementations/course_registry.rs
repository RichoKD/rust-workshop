use std::collections::BTreeMap;

use crate::student_registry_project::{
    types::basic_types::{Course, CourseRegistry},
    utils::convert_to_string,
};

// CourseRegistry implementation
impl CourseRegistry {
    // static method to initializea a new CourseRegistry
    pub fn new() -> CourseRegistry {
        CourseRegistry {
            courses: BTreeMap::new(),
            total_courses: 0,
        }
    }

    // method to add a course to the registry
    pub fn add_course(&mut self, name: String, max_capacity: u32) -> u32 {
        let course_id = self.total_courses + 1;

        let course = Course::new(course_id, name, max_capacity);

        self.total_courses += 1;
        self.courses.insert(course_id, course);

        course_id
    }

    // method to get a course by id
    pub fn get_course_by_id(&mut self, id: u32) -> Option<&mut Course> {
        self.courses.get_mut(&id)
    }
}

// Course implementation
impl Course {
    // static method to initialize a new course
    pub fn new(id: u32, name: String, max_capacity: u32) -> Course {
        Course {
            id,
            name,
            student_ids: Vec::new(),
            max_capacity,
        }
    }

    // method to add a student to a course
    pub fn add_student(&mut self, student_id: u32) -> Result<(), String> {
        // checks if the course has reached max capacity, if it has, returns an error
        if self.student_ids.len() >= self.max_capacity as usize {
            return Err(convert_to_string("Course is full"));
        }
        // checks if the student is already registered for the course
        let student = self.student_ids.iter().find(|&s| s == &student_id);

        match student {
            // if student is already registered, returns an error
            Some(_) => Err(convert_to_string("Student already registered")),

            // if student is not registered, adds the student to the course
            None => {
                self.student_ids.push(student_id);
                Ok(())
            }
        }
    }
}
