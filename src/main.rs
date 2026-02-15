pub mod image;

use axum::{Router, routing::get, extract::DefaultBodyLimit};
use image::image_controller::image_routes;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with nice formatting
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pompom=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting pompom server");

    let app = Router::new()
        .route("/", get(|| async { "hi" }))
        .nest("/image", image_routes())
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); // 50MB limit

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
