mod chars_bool_and_unit;
mod compound_types;
mod exercises;
mod functions;
mod integers;
mod ownership;
mod referencing_and_borrowing;
mod statements_and_expressions;

pub mod student_registry_project;

use student_registry_project::{
    types::basic_types::{Sex, Student, StudentRegistry},
    utils::convert_to_string,
};

fn main() {
    // exercises::ex_1::unused_var();
    // exercises::ex_2::mutate_variable();
    // exercises::ex_3::scope_fn();
    // exercises::ex_4::shadow_fn();
    // exercises::ex_4::shadow_fn_b();
    // exercises::ex_5::unused_var();
    // exercises::ex_6::destructure();
    // exercises::ex_7::destructure_assignment();
    // integers::ex_1::int_ex_1();
    // integers::ex_2::int_ex_2();
    // integers::ex_3::int_ex_3();
    // integers::ex_4::int_ex_4();
    // integers::ex_5::int_ex_5();
    // integers::ex_6::int_ex_6();
    // chars_bool_and_unit::ex_1::ex_1();
    // chars_bool_and_unit::ex_2::ex_2();
    // chars_bool_and_unit::ex_3::ex_3();
    // chars_bool_and_unit::ex_4::main();
    // chars_bool_and_unit::ex_5::main();
    // chars_bool_and_unit::ex_6::main();
    // statements_and_expressions::ex_1::main(); // wip
    // statements_and_expressions::ex_2::main();
    // statements_and_expressions::ex_3::main();
    //   functions::ex_1::main();
    // functions::ex_2::main();
    // functions::ex_3::main(); // this is intended to panic
    // functions::ex_4::main(); // wip
    // ownership::ex_1::main();
    // ownership::ex_2::main();
    // ownership::ex_3::main();
    // ownership::ex_4::main();
    // ownership::ex_5::main();
    // ownership::ex_6::main();
    // ownership::ex_7::main();
    // ownership::ex_8::main();
    // ownership::ex_9::main();
    // referencing_and_borrowing::ex_1::main();
    // referencing_and_borrowing::ex_2::main();
    // referencing_and_borrowing::ex_3::main();
    // referencing_and_borrowing::ex_4::main();
    // referencing_and_borrowing::ex_5::main();
    // referencing_and_borrowing::ex_6::main();
    // referencing_and_borrowing::ex_7::main();
    // referencing_and_borrowing::ex_8::main();
    // referencing_and_borrowing::ex_9::main();
    // referencing_and_borrowing::ex_10::main();
    // referencing_and_borrowing::ex_11::main();
    // compound_types::ex_1::main();
    // compound_types::ex_2::main();
    // compound_types::ex_3::main();
    // compound_types::ex_4::main();
    // compound_types::ex_5::main();
    // compound_types::ex_6::main();
    // compound_types::ex_7::main();
    // compound_types::ex_8::main();
    // compound_types::ex_9::main();
    // compound_types::ex_10::main();
    // compound_types::ex_11::main();
    // compound_types::ex_12::main();

    // 09-08-2024 session
    // Student Registry
    let mut rust_cohort = StudentRegistry::new_session();
    let _stephanie = rust_cohort.register(
        convert_to_string("Stephanie"),
        convert_to_string("Nwankwo"),
        20,
        5.8,
        Sex::Female,
    );

    // registers a second student to the cohort
    let _rico = rust_cohort.register(
        convert_to_string("rico"),
        convert_to_string("blockheader"),
        22,
        6.1,
        Sex::Male,
    );
    // println!("stephanie as student: {:#?} ", stephanie);
    // println!("stephanie as student: {stephanie:#?} ");

    // student with id 0 is stephanie
    let st_1 = rust_cohort.get_student_by_id(0);
    println!("student with id 0 is: {st_1:#?} ");

    // student with id 1 is rico
    let st_2 = rust_cohort.get_student_by_id(1);
    println!("student with id 1 is: {st_2:#?} ");
    print!("\n\n");

    // create course 1 with max capacity of 1 student
    let course_1 = rust_cohort.add_course(convert_to_string("Introduction To Rust"), 1);

    // create course 2 with max capacity of 3 students
    let course_2 = rust_cohort.add_course(convert_to_string("Advance Cairo"), 3);

    println!("course 1 is created with id: {course_1:#?}");
    println!("course 2 is created with id: {course_2:#?}");
    print!("\n\n");

    // attempt to register stephanie for course 1
    // stephanie student id is 0
    let course_1_reg_result_1 = rust_cohort.register_student_for_course(course_1, 0);

    // attempt to register rico for course 1
    // rico student id is 0
    // registration should return an error because course 1 has max capacity of 1 student
    let course_1_reg_result_2 = rust_cohort.register_student_for_course(course_1, 1);

    println!("stephanie registration result for course_1: {course_1_reg_result_1:#?}");
    println!("rico registration result for course_2: {course_1_reg_result_2:#?}");
    print!("\n\n");

    // attempt to register stephanie for course 1
    // stephanie student id is 0
    // also test that student can register for more than 1 course
    let course_2_reg_result_1 = rust_cohort.register_student_for_course(course_2, 0);

    // attempt to register stephanie for course 1 twice
    // stephanie student id is 0
    // should fail as double registration are not allowed
    let course_2_reg_result_2 = rust_cohort.register_student_for_course(course_2, 0);

    // attempt to register rico for course 1
    // rico student id is 0
    // should return an Ok result since max capacity is not reached
    let course_2_reg_result_3 = rust_cohort.register_student_for_course(course_2, 1);

    println!("stephanie registration result for course_2: {course_2_reg_result_1:#?}");
    println!("stephanie second registration result for course_2: {course_2_reg_result_2:#?}");
    println!("rico registration result for course_2: {course_2_reg_result_3:#?}");
}
