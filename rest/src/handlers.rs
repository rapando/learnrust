use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};


use crate::entities;
use crate::models;

pub async fn home_handler() -> &'static str {
    "Home Handler"
}

pub async fn create_user_handler() -> impl IntoResponse {
    println!("received request to create user");
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("user created successfully"))
        .unwrap()
}

pub async fn list_users_handler() -> Json<Vec<entities::User>> {
    println!("received request to list users");
    let users = models::fetch_users();
    Json::from(users)
}
