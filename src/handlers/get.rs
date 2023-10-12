use axum::http;

pub async fn health() -> (http::StatusCode, &'static str) {
    (http::StatusCode::OK, "Hello, World!")
}
