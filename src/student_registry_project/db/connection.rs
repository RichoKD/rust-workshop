use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub async fn get_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn import_db_pool() -> PgPool {
    get_db_pool().await
}
