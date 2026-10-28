use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

struct AppState {
    index_html: Bytes,
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .compact()
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();

    // Load HTML once at startup.
    let html = tokio::fs::read("templates/index.html")
        .await
        .expect("Failed to load templates/index.html");

    let state = Arc::new(AppState {
        index_html: Bytes::from(html),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/api", get(hello_world))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG)),
        )
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 3000");

    tracing::info!("Server listening on http://{addr}");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn index(State(state): State<Arc<AppState>>) -> Html<Bytes> {
    // Bytes clones cheaply without copying the HTML contents.
    Html(state.index_html.clone())
}

async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Hello world"
        })),
    )
}
