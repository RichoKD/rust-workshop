use crate::models::student::Student;
use crate::auth::password::hash_password;
use sqlx::{PgPool, Error};

pub async fn get_all_students(pool: &PgPool) -> Result<Vec<Student>, Error> {
    let students = sqlx::query_as!(
        Student,
        r#"
        SELECT id, first_name, last_name, age, height, sex as "sex: _"
        FROM students
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(students)
}

pub async fn get_student_by_id(pool: &PgPool, id: u32) -> Result<Student, Error> {
    let student = sqlx::query_as!(
        Student,
        r#"
        SELECT id, first_name, last_name, age, height, sex as "sex: _"
        FROM students
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(student)
}

pub async fn create_student(pool: &PgPool, student: Student) -> Result<(), Error> {

    let password_hash = hash_password(&student.password);

    sqlx::query!(
        r#"
        INSERT INTO students (id, first_name, last_name, age, height, sex, password)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        student.id,
        student.first_name,
        student.last_name,
        student.age,
        student.height,
        student.sex as _,
        password_hash

    )
    .execute(pool)
    .await?;

    Ok(())
}
