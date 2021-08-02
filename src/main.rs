//! Run with
//! ```not_rust
//! RUST_LOG=info cargo run
//! ```
use axum::prelude::*;
use serde::Serialize;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Setup tracing logging.
    tracing_subscriber::fmt::init();

    let middleware_stack = ServiceBuilder::new()
        .timeout(Duration::from_secs(15))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .into_inner();

    let app = route("/", get(handler)).layer(middleware_stack);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    let server = hyper::Server::bind(&addr).serve(app.into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {}", err);
    };
}

async fn handler() -> response::Json<Hello> {
    response::Json(Hello {
        hello: "world".to_string(),
    })
}

#[derive(Serialize)]
struct Hello {
    hello: String,
}
