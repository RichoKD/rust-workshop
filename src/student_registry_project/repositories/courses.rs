// use crate::models::course::Course;
// use sqlx::{PgPool, Error};

// pub async fn get_all_courses(pool: &PgPool) -> Result<Vec<Course>, Error> {
//     let courses = sqlx::query_as!(
//         Course,
//         r#"
//         SELECT id, name, student_count, max_capacity
//         FROM courses
//         "#
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(courses)
// }

// pub async fn get_course_by_id(pool: &PgPool, id: u32) -> Result<Course, Error> {
//     let course = sqlx::query_as!(
//         Course,
//         r#"
//         SELECT id, name, student_count, max_capacity
//         FROM courses
//         WHERE id = $1
//         "#,
//         id
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(course)
// }

// pub async fn create_course(pool: &PgPool, course: Course) -> Result<(), Error> {
//     sqlx::query!(
//         r#"
//         INSERT INTO courses (id, name, student_count, max_capacity)
//         VALUES ($1, $2, $3, $4)
//         "#,
//         course.id,
//         course.name,
//         0,
//         course.max_capacity
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }

// // enroll a student to a course
// pub async fn enroll_student_to_course(pool: &PgPool, course_id: u32, student_id: u32) -> Result<(), Error> {
//     sqlx::transaction(pool, |tx| async move {
//         sqlx::query!(
//             r#"
//             UPDATE courses
//             SET student_count = student_count + 1
//             WHERE id = $1
//             "#,
//             course_id
//         )
//         .execute(tx)
//         .await?;

//         sqlx::query!(
//             r#"
//             INSERT INTO student_courses (course_id, student_id)
//             VALUES ($1, $2)
//             "#,
//             course_id,
//             student_id
//         )
//         .execute(tx)
//         .await?;

//         Ok(())
//     })
//     .await
// }

// // get students enrolled in a course
// pub async fn get_students_enrolled_in_course(pool: &PgPool, course_id: u32) -> Result<Vec<u32>, Error> {
//     let students = sqlx::query!(
//         r#"
//         SELECT student_id
//         FROM student_courses
//         WHERE course_id = $1
//         "#,
//         course_id
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(students.iter().map(|s| s.student_id).collect())
// }
