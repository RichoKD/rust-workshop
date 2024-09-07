use crate::student_registry_project::models::basic_types::{
    CourseRegistry, Sex, Student, StudentRegistry,
};
use crate::student_registry_project::utils::convert_to_string;

impl StudentRegistry {
    // this initializes a new StudentRegistry
    // new array of students
    pub fn new_session() -> StudentRegistry {
        StudentRegistry {
            total_students: Vec::new(),
            course_registry: CourseRegistry::new(),
        }
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
        };
        self.total_students.push(student.clone());
        student
        // student_id
    }

    // util function to get student by id
    pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
        self.total_students.get(id as usize)
    }

    // util function to register student for a course
    pub fn register_student_for_course(
        &mut self,
        course_id: u32,
        student_id: u32,
    ) -> Result<(), String> {
        // checks if the student with the id exists, this way, can't add students that don't exist
        let student = self.get_student_by_id(student_id);

        match student {
            // if student exists, adds the student to the course
            Some(_) => {
                // another check to see if the course exists
                let course = self.course_registry.courses.get_mut(&course_id);

                match course {
                    // tries to add the student to the course
                    Some(found_course) => {
                        found_course.add_student(student_id)?;
                        Ok(())
                    }
                    // if course doesn't exist, returns an error
                    None => Err(convert_to_string("Course with the provided Id")),
                }
            }
            // if student doesn't exist, returns an error
            None => Err(convert_to_string("Student with the provided Id not found")),
        }
    }

    // util method to add a course
    pub fn add_course(&mut self, name: String, max_capacity: u32) -> u32 {
        self.course_registry.add_course(name, max_capacity)
    }
}
