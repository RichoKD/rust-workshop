use crate::student_registry_project::models::student_model::Student;
use crate::student_registry_project::auth::password::hash_password;
use sqlx::Error;
use crate::student_registry_project::db::database::{Database, DatabaseTrait};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct StudentRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait StudentRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn find_by_email(&self, email: String) -> Option<Student>;
    async fn find(&self, id: i32) -> Result<Student, Error>;
}


#[async_trait]
impl StudentRepositoryTrait for StudentRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
    
    async fn find_by_email(&self, email: String) -> Option<Student> {
        let user = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE email = ?")
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        return user;
    }

    async fn find(&self, id: i32) -> Result<Student, Error> {
        let user = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await;
        return user;
    }
}
















/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// let db_pool = connection::import_db_pool().await;


// pub async fn get_students(
//     State(db_pool): State<PgPool>,
//   ) ->  Result<Vec<Student>, Error> {
//     let rows = sqlx::query_as!(Student, "SELECT * FROM students")
//       .fetch_all(&db_pool)
//       .await
//       .map_err(|e| {
//         (
//           StatusCode::INTERNAL_SERVER_ERROR,
//           json!({"success": false, "message": e.to_string()}).to_string(),
//         )
//       })?;
  
//     rows
// }

// async fn delete_task(
//   State(db_pool): State<PgPool>,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//   sqlx::query!("DELETE FROM tasks_z WHERE task_id = $1", task_id,)
//     .execute(&db_pool)
//     .await
//     .map_err(|e| {
//       (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         json!({"success": false, "message": e.to_string()}).to_string(),
//       )
//     })?;

//   Ok((StatusCode::OK, json!({"success":true}).to_string()))
// }



// pub async fn get_all_students(db_pool: &PgPool) -> Result<Vec<Student>, Error> {

//     let students = sqlx::query_as!(
//         Student,
//         r#"
//         SELECT id, first_name, last_name, age, height, sex
//         FROM students
//         "#
//     )
//     .fetch_all(db_pool)
//     .await?;

//     Ok(students)
// }

// pub async fn get_student_by_id(pool: &PgPool, id: u32) -> Result<Student, Error> {
//     let student = sqlx::query_as!(
//         Student,
//         r#"
//         SELECT id, first_name, last_name, age, height, sex
//         FROM students
//         WHERE id = $1
//         "#,
//         id
//     )
//     .fetch_optional(pool)
//     .await?
//     .ok_or_else(|| Error::RowNotFound)?;

//     Ok(student)
// }

// pub async fn create_student(pool: &PgPool, student: Student) -> Result<(), Error> {
//     let password_hash = hash_password(&student.password);

//     sqlx::query!(
//         r#"
//         INSERT INTO students (id, first_name, last_name, age, height, sex, password)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//         "#,
//         student.id,
//         student.first_name,
//         student.last_name,
//         student.age,
//         student.height,
//         student.sex as _,
//         password_hash
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }

