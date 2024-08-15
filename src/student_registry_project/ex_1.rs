// use student_registry_project::{
use crate::student_registry_project::types::basic_types::{Sex,Course, Student, StudentRegistry};

// use types::basic_types::{Sex, Student, StudentRegistry};
// use utils::convert_to_string;
// };

use std::io;
use std::io::Write; // for flush

// fn create_course(name: String, capacity: u32, mut _courses: Vec<Course>) -> Vec<Course> {
//     let course_id: u32 = _courses.len() as u32 + 1;
//     let new_course = Course {
//         id: course_id,
//         name: name,
//         capacity: capacity,
//         enrolled_student_ids: Vec::new(),
//     };
//     _courses.push(new_course);
//     _courses
// }

// fn prompt(input: &str) {
//     print!("{input}");
//     io::stdout().flush().unwrap();
    
// }
fn get_courses(rust_cohort: &mut StudentRegistry) -> u32 {
    print!("{:#?}", rust_cohort.course_registry);
    io::stdout().flush().unwrap();

    print!("Enter course id: \n");
    io::stdout().flush().unwrap();
    let mut course_id = String::new();
    io::stdin()
        .read_line(&mut course_id)
        .expect("Failed to read line");
    let course_id = course_id
        .trim()
        .parse::<u32>()
        .expect("Invalid input. Please enter a number.");
    // let course = rust_cohort.get_course_by_id(course_id);
    // println!("course: {course:#?}");
    course_id
}

fn student_flow(rust_cohort: &mut StudentRegistry) {
    let mut student_state: i32 = 0;
    loop {
        let mut input = String::new();

        print!("1. Add student\n2. Get student\n3.Enroll student in course\n4.Return\n");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        student_state = input
            .trim()
            .parse::<i32>()
            .expect("Invalid input. Please enter a number.");

        // Add student
        if student_state == 1 {
            print!("Enter first name: ");
            io::stdout().flush().unwrap();
            let mut first_name = String::new();
            io::stdin()
                .read_line(&mut first_name)
                .expect("Failed to read line");
            let first_name = first_name.trim().to_string();

            print!("Enter last name: ");
            io::stdout().flush().unwrap();
            let mut last_name = String::new();
            io::stdin()
                .read_line(&mut last_name)
                .expect("Failed to read line");
            let last_name = last_name.trim().to_string();

            print!("Enter age: ");
            io::stdout().flush().unwrap();
            let mut age = String::new();
            io::stdin()
                .read_line(&mut age)
                .expect("Failed to read line");
            let age = age
                .trim()
                .parse::<u8>()
                .expect("Invalid input. Please enter a number.");

            print!("Enter height: ");
            io::stdout().flush().unwrap();
            let mut height = String::new();
            io::stdin()
                .read_line(&mut height)
                .expect("Failed to read line");
            let height = height
                .trim()
                .parse::<f32>()
                .expect("Invalid input. Please enter a number.");

            print!("Enter sex: 1 for male, 2 for female: ");
            let mut sex = String::new();
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut sex)
                .expect("Failed to read line");
            let sex = sex.trim().parse::<u8>().unwrap();

            if sex == 1 {
                let student = rust_cohort.register(first_name.clone(), last_name.clone(), age, height, Sex::Male);
                println!("student: {student:#?}");
                io::stdout().flush().unwrap();
                // Get course_id and enroll student
                let course_id = get_courses(rust_cohort);
                rust_cohort.enroll_student_in_course(student.id, course_id);
            } else if sex == 2 {
                let student = rust_cohort.register(first_name.clone(), last_name.clone(), age, height, Sex::Female);
                println!("student: {student:#?}");
                io::stdout().flush().unwrap();
                // Get course_id and enroll student
                let course_id = get_courses(rust_cohort);
                rust_cohort.enroll_student_in_course(student.id, course_id);
            } else {
                println!("Invalid input");
                io::stdout().flush().unwrap();
                
                continue;
            }

            // println!("Student {first_name} {last_name}  added successfully");
            // io::stdout().flush().unwrap();

        }

        // Get student
        if student_state == 2 {
            let length = rust_cohort.total_students.len() as u32;
            if length == 0 {
                println!("No students in the cohort");
                io::stdout().flush().unwrap();

            } else {
                let len = length - 1;
                print!("Enter student id(Range 0 to {len}): ");
                io::stdout().flush().unwrap();

                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().unwrap();
                let student = rust_cohort.get_student_by_id(id);
                println!("student: {student:#?}");
            }
        }
        // Enroll student in course
        if student_state == 3{
            // print students
            print!("{:#?}", rust_cohort.total_students);
            io::stdout().flush().unwrap();

            print!("Enter student id: ");
            io::stdout().flush().unwrap();

            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id = id.trim().parse::<u32>().unwrap();

            let course_id = get_courses(rust_cohort);
            rust_cohort.enroll_student_in_course(id, course_id);
        }
        // Exit
        if student_state == 4 {
            break;
        }
    }  
}

fn create_course(rust_cohort: &mut StudentRegistry) {
    // create course
    print!("Enter course name: ");
    io::stdout().flush().unwrap();
    let mut course_name = String::new();
    io::stdin()
        .read_line(&mut course_name)
        .expect("Failed to read line");
    let course_name = course_name.trim().to_string();
    print!("Enter course capacity: ");
    io::stdout().flush().unwrap();

    let mut course_capacity = String::new();
    io::stdin()
        .read_line(&mut course_capacity)
        .expect("Failed to read line");
    let course_capacity = course_capacity
        .trim()
        .parse::<u32>()
        .expect("Invalid input. Please enter a number.");
    rust_cohort.add_course(course_name, course_capacity);

    // rust_cohort
}

pub fn main() {
    let mut rust_cohort = StudentRegistry::new_session();
    // let mut courses: Vec<Course> = Vec::new();
    // rust_cohort.add_course("Rust".to_string(), 2);
    // let courses = rust_cohort.course_registry.clone();
    // println!("courses: {courses:#?}");

    // print courses
    // println!("courses: {rust_cohort:#?}");
    

    let mut main_state: i32 = 0;

    loop{
        if rust_cohort.course_registry.len() as u32 == 0{

            print!("1. Add course\n");
            io::stdout().flush().unwrap();

            let mut input  = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            main_state = input
                .trim()
                .parse::<i32>()
                .expect("Invalid input. Please enter a number.");

            if main_state == 1 {
                create_course(&mut rust_cohort);
                // break;
            }else{
                continue;
            }
        }else {
            print!("1. Add course\n2. Manage students\n3. Exit\n");
            io::stdout().flush().unwrap();

            let mut input  = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            main_state = input
                .trim()
                .parse::<i32>()
                .expect("Invalid input. Please enter a number.");

            if main_state == 1 {
                create_course(&mut rust_cohort);

            }else if main_state == 2 {
                student_flow(&mut rust_cohort);

            }else if main_state == 3 {
                println!("Bye!");
                break;
            }else{
                continue;
            }
        }
    }
}

pub fn mainz() {
    // Student Registry
    let mut rust_cohort = StudentRegistry::new_session();
    // let mut courses: Vec<Course> = Vec::new();

    let mut state: i32 = 0;

    // courses = create_course("Rust".to_string(), 2, courses);

    // Cli state machine
    loop {
        let mut input = String::new();

        print!("1. Add student\n2. Get student\n3. Exit\n");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        state = input
            .trim()
            .parse::<i32>()
            .expect("Invalid input. Please enter a number.");

        // Add student
        if state == 1 {
            print!("Enter first name: ");
            io::stdout().flush().unwrap();
            let mut first_name = String::new();
            io::stdin()
                .read_line(&mut first_name)
                .expect("Failed to read line");
            let first_name = first_name.trim().to_string();

            print!("Enter last name: ");
            io::stdout().flush().unwrap();
            let mut last_name = String::new();
            io::stdin()
                .read_line(&mut last_name)
                .expect("Failed to read line");
            let last_name = last_name.trim().to_string();

            print!("Enter age: ");
            io::stdout().flush().unwrap();
            let mut age = String::new();
            io::stdin()
                .read_line(&mut age)
                .expect("Failed to read line");
            let age = age
                .trim()
                .parse::<u8>()
                .expect("Invalid input. Please enter a number.");

            print!("Enter height: ");
            io::stdout().flush().unwrap();
            let mut height = String::new();
            io::stdin()
                .read_line(&mut height)
                .expect("Failed to read line");
            let height = height
                .trim()
                .parse::<f32>()
                .expect("Invalid input. Please enter a number.");

            print!("Enter sex: 1 for male, 2 for female: ");
            let mut sex = String::new();
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut sex)
                .expect("Failed to read line");
            let sex = sex.trim().parse::<u8>().unwrap();

            if sex == 1 {
                let student = rust_cohort.register(first_name.clone(), last_name.clone(), age, height, Sex::Male);
                println!("student: {student:#?}");
            } else if sex == 2 {
                let student = rust_cohort.register(first_name.clone(), last_name.clone(), age, height, Sex::Female);
                println!("student: {student:#?}");
            } else {
                println!("Invalid input");
                io::stdout().flush().unwrap();
                
                continue;
            }

            // println!("Student {first_name} {last_name}  added successfully");
            // io::stdout().flush().unwrap();

        }

        // Get student
        if state == 2 {
            let length = rust_cohort.total_students.len() as u32;
            if length == 0 {
                println!("No students in the cohort");
                io::stdout().flush().unwrap();

            } else {
                let len = length - 1;
                print!("Enter student id(Range 0 to {len}): ");
                io::stdout().flush().unwrap();

                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().unwrap();
                let student = rust_cohort.get_student_by_id(id);
                println!("student: {student:#?}");
            }
        }
        // Exit
        if state == 3 {
            println!("Bye!");
            break;
        }
    }    
}