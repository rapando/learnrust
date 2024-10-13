use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;


#[derive(Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
    description: String
}

async fn create_item(pool: web::Data<PgPool>, item: web::Json<Item>) -> impl Responder {
    let item = item.into_inner();
    let result = sqlx::query(
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id",
        item.name,
        item.description,
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    let items = sqlx::query_as!(Item, "SELECT id, name, description FROM items")
        .fetch_all(pool.get_ref())
        .await;

    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(get_items))
    })
    .bind("0.0.0.0:8080")?
        .run()
        .await
}
