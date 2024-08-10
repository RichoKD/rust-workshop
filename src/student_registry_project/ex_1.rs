// use student_registry_project::{
use crate::student_registry_project::types::basic_types::{Sex, Student, StudentRegistry};

// use types::basic_types::{Sex, Student, StudentRegistry};
// use utils::convert_to_string;
// };

use std::io;
use std::io::Write; // for flush


pub fn main() {
    // Student Registry
    let mut rust_cohort = StudentRegistry::new_session();

    let mut state: i32 = 0;

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