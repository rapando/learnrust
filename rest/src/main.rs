mod entities;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let router = routes::create_router();

    println!("starting server on port 3000");
    // start the axum server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
