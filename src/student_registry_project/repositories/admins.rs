use crate::student_registry_project::models::admin_model::Admin;
use crate::student_registry_project::auth::password::{hash_password, verify_password};
use crate::student_registry_project::auth::jwt;
use sqlx::Error;
use crate::student_registry_project::db::database::{Database, DatabaseTrait};
use async_trait::async_trait;
use std::sync::Arc;



#[derive(Clone)]
pub struct AdminRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait AdminRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn login(&self, email: String, password: String) -> Option<String>;
    async fn find_by_email(&self, email: String) -> Option<Admin>;
    async fn find(&self, id: i32) -> Result<Admin, Error>;
}


#[async_trait]
impl AdminRepositoryTrait for AdminRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn login(&self, email: String, password: String) -> Option<String> {
        let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE email = ?")
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        if let Some(admin) = admin {
            if let Ok(result) = verify_password(&admin.password, &password){
                let token = jwt::create_jwt(admin.id.to_string());
                return Some(token);
            }
        }
        return None;
    }
    
    async fn find_by_email(&self, email: String) -> Option<Admin> {
        let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE email = ?")
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None);
        return admin;
    }

    async fn find(&self, id: i32) -> Result<Admin, Error> {
        let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE id = ?")
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await;
        return admin;
    }
}