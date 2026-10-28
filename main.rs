use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::io;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/api", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started successfully on 0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}


async fn index() -> Html<String> {
    let html = tokio::fs::read_to_string("templates/index.html")
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to read index.html: {err}");
            "<h1>500 - Internal Server Error</h1>".to_string()
        });

    Html(html)
}


async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello world"
    });

    Json(json_response)
}

