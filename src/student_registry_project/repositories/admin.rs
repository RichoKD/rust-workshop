use crate::models::admin::Admin;
use sqlx::{PgPool, Error};

pub async fn get_all_admins(pool: &PgPool) -> Result<Vec<Admin>, Error> {
    let admins = sqlx::query_as!(
        Admin,
        r#"
        SELECT id, first_name, last_name
        FROM admins
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(admins)
}


pub async fn get_admin_by_id(pool: &PgPool, id: u32) -> Result<Admin, Error> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        SELECT id, first_name, last_name
        FROM admins
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(admin)
}


// login