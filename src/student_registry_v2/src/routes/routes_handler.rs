// use axum::http::{Request, Response, StatusCode};
// use axum::response::IntoResponse;
use axum::{routing::{post, get}, Json, Router};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
// use crate::utils;
use crate::types::basic_types::{ student_registry::StudentRegistry, course::Course, course_registry::CourseRegistry };

async fn say_gm() -> &'static str {
    "GM"
}

async fn hello_ab() -> &'static str {
    "AB"
}


pub fn routes() -> Router {
    // add CORS layer to allow any origin
    // let cors = utils::cors_handler();

    // integrate CORS to router
    Router::new()
        .route("/", get(say_gm))
        .route("/ab", get(hello_ab))
        // .layer(cors)
}