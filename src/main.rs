pub mod image;

use axum::{Router, routing::get, extract::DefaultBodyLimit};
use image::image_controller::image_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hi" }))
        .nest("/image", image_routes())
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); // 50MB limit

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
