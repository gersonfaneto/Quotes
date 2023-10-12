use axum::{extract, http};
use sqlx::PgPool;

use crate::models::quote::Quote;

pub async fn health() -> (http::StatusCode, &'static str) {
    (http::StatusCode::OK, "Hello, World!")
}

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
