use axum::{
    Router,
    extract::Multipart,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use tracing::{info, warn, debug, instrument};

use super::resize::resize_image;

/// Image processing router
pub fn image_routes() -> Router {
    Router::new().route("/resize", post(resize_image_controller))
}

/// resize image endpoint
#[instrument(skip(multipart))]
pub async fn resize_image_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Received image resize request");

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("<unnamed>");
        debug!(field_name = %field_name, "Found multipart field");

        if field.name() == Some("image") {
            debug!("Processing image field");
            let data = field.bytes().await.map_err(|e| {
                warn!(error = ?e, "Failed to read field bytes");
                StatusCode::BAD_REQUEST
            })?;

            let data_size = data.len();
            info!(bytes = data_size, "Read image data");

            let resized_bytes = resize_image(data).await?;

            info!(
                original_bytes = data_size,
                resized_bytes = resized_bytes.len(),
                "Successfully resized image"
            );

            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/jpg")],
                resized_bytes,
            ));
        }
    }

    warn!("No 'image' field found in multipart data");
    Err(StatusCode::BAD_REQUEST)
}
