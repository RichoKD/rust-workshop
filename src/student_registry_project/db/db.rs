use std::time::Duration;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing::{get, patch},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

// dotenvy::dotenv().expect("Unable to access .env file");

pub fn create_db_pool() -> PgPool {
    const server_address:String = match std::env::var("SERVER_ADDRESS") {
        Ok(val) => val,
        Err(_) => "127.0.0.1:3000".to_owned(),
    };
    
    const database_url:String = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => panic!("DATABASE_URL not found in env file"),
    };
    
    //set variables from enviroment variables
    // let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    const database_url:String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");
    
    //create our database pool
    let db_pool = PgPoolOptions::new()
      .max_connections(64)
      .acquire_timeout(Duration::from_secs(5))
      .connect(&database_url)
      .await
      .expect("can't connect to database");

    db_pool
    
    //create our tcp listener
    // let listener = TcpListener::bind(server_address)
    //   .await
    //   .expect("Could not create tcp listener");
    
    // println!("listening on {}", listener.local_addr().unwrap());
}
  //set variables from enviroment variables


// pub fn 