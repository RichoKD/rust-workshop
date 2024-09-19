use crate::student_registry_project::handlers::course_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    let router = Router::new().route("/profile", get(profile_handler::profile));
    return router;
}