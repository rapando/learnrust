use axum::{routing, Router};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", routing::get(handlers::home_handler))
        .route("/users", routing::post(handlers::create_user_handler))
        .route("/users", routing::get(handlers::list_users_handler))
}
