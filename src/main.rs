mod handlers;
mod models;

use axum::routing::{delete, get, post, put, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("Missing enviromet variable - DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let res = sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(handlers::get::health))
        .route("/quotes", post(handlers::post::create_quote))
        .route("/quotes", get(handlers::get::read_quotes))
        .route("/quotes/:id", put(handlers::put::update_quote))
        .route("/quotes/:id", delete(handlers::delete::delete_quote))
        .with_state(pool);

    println!("Listening on {address}...");

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
