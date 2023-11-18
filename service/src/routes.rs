use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, routing::*, Router,
};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(hello))
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))
        .fallback(handler_404)
}

async fn hello() -> String {
    "Hello, World!".to_string()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found".to_string())
}
