use crate::student_registry_project::types::basic_types::{Sex, Student, StudentRegistry};

impl StudentRegistry {
    // this initializes a new StudentRegistry
    // new array of students
    pub fn new_session() -> StudentRegistry {
        StudentRegistry {
            total_students: Vec::new(),
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
}
