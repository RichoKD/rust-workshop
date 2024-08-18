use crate::student_registry_project::types::basic_types::{Sex, Course, Student, StudentRegistry};
use std::collections::HashMap;

impl StudentRegistry {
    // this initializes a new StudentRegistry
    // new array of students
    pub fn new_session() -> StudentRegistry {
        StudentRegistry {
            total_students: Vec::new(),
            course_registry: Vec::new(),
        }
    }

    // add course
    pub fn add_course(&mut self, _name: String, _capacity: u32) {
        let course_id: u32 = self.course_registry.len() as u32 + 1;
        // let _enrolled_student_ids: Vec<u32> = Vec::with_capacity(_capacity as usize);
        let _enrolled_student_ids: HashMap<u32, String> = HashMap::with_capacity(_capacity as usize);


        self.course_registry.push(Course {
            id: course_id,
            name: _name,
            capacity: _capacity,
            enrolled_student_ids: _enrolled_student_ids,
        });
    }

    // register student
    pub fn register(
        &mut self,
        f_name: String,
        l_name: String,
        _age: u8,
        _height: f32,
        _sex: Sex,
    ) -> Student {
        // generate a unique ID
        let student_id: u32 = self.total_students.len() as u32 + 1;

        let student = Student {
            first_name: f_name,
            last_name: l_name,
            id: student_id,
            age: _age,
            height: _height,
            sex: _sex,
            enrolled_course_ids: HashMap::new(),
        };
        self.total_students.push(student.clone());
        student
        // student_id
    }

    // util function to get student by id
    pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
        self.total_students.get(id as usize)
    }


    // Method to enroll a student in a course
    pub fn enroll_student_in_course(&mut self, student_id: u32, course_id: u32) {
        // Find the student and course
       
        let student_index: usize = (student_id - 1).try_into().unwrap();
        let course_index: usize = (course_id - 1).try_into().unwrap();

        // Check if the student exists
        if student_index >= self.total_students.len() {
            println!("Student not found.");
            return;
        }

        // Check if the course exists
        if course_index >= self.course_registry.len() {
            println!("Course not found.");
            return;
        }

        // Check if the student is already enrolled in the course
        if self.total_students[student_index]
            .enrolled_course_ids
            .get(&course_id)
            .is_some()
        {
            println!("Student is already enrolled in the course.");
            return;
        }


        // Check if the course has available capacity
        if self.course_registry[course_index].enrolled_student_ids.len() < self.course_registry[course_index].capacity as usize {
            // Add the student to the course's enrolled students
            self.course_registry[course_index].enrolled_student_ids
                .insert(student_id, self.total_students[student_index].first_name.clone());

            // Add the course to the student's enrolled courses
            self.total_students[student_index].enrolled_course_ids
                .insert(course_id, self.course_registry[course_index].name.clone());

            println!("Student enrolled in course.");

        } else {
            println!("Course is full. Cannot enroll student.");
        }
    }
}
