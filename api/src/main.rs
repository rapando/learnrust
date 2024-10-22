use axum::{
    extract::{Path, Request}, 
    http::StatusCode, 
    middleware::{ from_fn, Next},
    response::{IntoResponse, Json, Response},
    routing::get, 
    Extension, 
    Router,
};
use std::{sync::Arc, time::{ Instant}};
use thiserror::Error;
use serde_json::{Value, json};
use tower::ServiceBuilder;


// define our custom errors
#[derive(Error, Debug)]
enum AppError {
    #[error("not found")]
    NotFound,
    #[error("internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

#[derive(Clone)]
struct State {
    app_version: String
}

// middleware
async fn logging_middleware(
    req: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();

    let response = next.run(req).await;

    let elapsed = start_time.elapsed();
    println!("request took  {:.2?}", elapsed);

    response
}


#[tokio::main]
async fn main() {
    let state = State{
        app_version: String::from("v0.0.1"),
    };

    // * When adding middleware you can put them in ServiceBuilder
    // * when a mw is applied using a layer, it applies to all endpoints after it

    let shared_state = Arc::new(state);
    let app = Router::new()
        .route("/", get(|| async {"hello, world"}))
        .route("/plain", get(plain_text))
        .route("/json", get(json))
        .route("/items/:id", get(handler_that_might_fail))
        .route("/state", get(handler_with_state))
        .layer(Extension(shared_state))
        // .layer(from_fn(logging_middleware))
        .layer(ServiceBuilder::new().layer(from_fn(logging_middleware)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({"data": 42}))
}

async fn handler_that_might_fail(Path(id): Path<u32>) -> Result<Json<serde_json::Value>, AppError> {
    if id == 1 {
        Ok(Json(json!({"item": "example"})))
    } else if id == 2 {
        Err(AppError::InternalServerError)
    } else {
        Err(AppError::NotFound)
    }
}

#[derive(serde::Serialize)]
struct VResponse {
    version: String,
}

async fn handler_with_state(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<VResponse>, AppError> {
    let response = VResponse{
        version: state.app_version.clone(),
    };
    Ok(Json(response))
}