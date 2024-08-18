use crate::student_registry_project::types::basic_types::{Sex, StudentRegistry};

use crate::student_registry_project::utils::{ input_any, input_f32, input_u32, input_u8};



fn get_courses(rust_cohort: &mut StudentRegistry) -> u32 {
    println!("{:#?}", rust_cohort.course_registry);

    let course_id = input_u32("Enter course id: ");
   
    course_id
}

fn student_flow(rust_cohort: &mut StudentRegistry) {

    loop {
        println!("1. Add student\n2. Get student\n3.Enroll student in course\n4.Return");

        let student_state = input_u8("Enter choice: ");

        match student_state {
            1 => {
                let (first_name, last_name, age, height, sex) = get_student_info();
                let student = rust_cohort.register(first_name, last_name, age, height, sex);
                println!("student: {:#?}", student);
                let course_id = get_courses(rust_cohort);
                rust_cohort.enroll_student_in_course(student.id, course_id);
            }
            2 => {
                if rust_cohort.total_students.is_empty() {
                    println!("No students in the cohort");
                } else {
                    let len = rust_cohort.total_students.len() - 1;
                    let id = input_u32(format!("Enter student id(Range 0 to {}): ", len).as_str());
                    if let Some(student) = rust_cohort.get_student_by_id(id) {
                        println!("student: {:#?}", student);
                    } else {
                        println!("Student not found");
                    }
                }
            }
            3 => {
                if rust_cohort.total_students.is_empty() {
                    println!("No students in the cohort");
                } else {
                    let len = rust_cohort.total_students.len() - 1;
                    let id = input_u32(format!("Enter student id(Range 0 to {}): ", len).as_str());
                    let course_id = get_courses(rust_cohort);
                    rust_cohort.enroll_student_in_course(id, course_id);
                }
            }
            4 => break,
            _ => continue,
        }
    }
}

fn get_student_info() -> (String, String, u8, f32, Sex) {
    let first_name = input_any("Enter first name: ");

    let last_name = input_any("Enter last name: ");

    let age = input_u8("Enter age: ");

    let height = input_f32( "Enter height: ");

    let sex = input_u8("Enter sex: 1 for male, 2 for female: ");
    let sex = match sex {
        1 => Sex::Male,
        2 => Sex::Female,
        _ => panic!("Invalid input. Please enter 1 for male or 2 for female."),
    };

    (first_name, last_name, age, height, sex)
}

fn create_course(rust_cohort: &mut StudentRegistry) {
    
    let course_name = input_any("Enter course name: ");

    let course_capacity = input_u32("Enter course capacity: ");
  
    rust_cohort.add_course(course_name, course_capacity);

}

pub fn main() {
    let mut rust_cohort = StudentRegistry::new_session();

    loop {

        if rust_cohort.course_registry.is_empty() {
            create_course(&mut rust_cohort);
        }

        println!("1. Add course\n2. Manage students\n3. Exit");

        let main_state = input_u8("Enter choice: ");

        match main_state {
            1 => create_course(&mut rust_cohort),
            2 => student_flow(&mut rust_cohort),
            3 => break,
            _ => continue,
        }
    }

    println!("Bye!");
}

