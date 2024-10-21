use axum::{
    routing::get,
    Router,
    response::{Json, IntoResponse},
    http::StatusCode,
    extract::Path,
    Extension,
};
use thiserror::Error;
use serde_json::{Value, json};
use tower_http::{trace::TraceLayer};
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

// application-wide state
#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(|| async {"hello, world"}))
        .route("/plain", get(plain_text))
        .route("/json", get(json))
        .route("/items/:id", get(handler_that_might_fail))
        .layer(
            ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(State{}))
        );

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
