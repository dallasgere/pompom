use axum::{
    Router,
    extract::Multipart,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use tracing::{debug, info, instrument, warn};

use super::image_service::resize_image;
use super::image_types::ResizeImageInput;

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

    let mut image_data = None;
    let mut width = None;
    let mut height = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("<unnamed>").to_string();
        debug!(field_name = %field_name, "Found multipart field");

        match field_name.as_str() {
            "image" => {
                image_data = Some(field.bytes().await.map_err(|e| {
                    warn!(error = ?e, "Failed to read field bytes");
                    StatusCode::BAD_REQUEST
                })?);
            }
            "width" => {
                let width_str = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get image width param");
                    StatusCode::BAD_REQUEST
                })?;

                width = Some(width_str.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse width as u32");
                    StatusCode::BAD_REQUEST
                })?);
            }
            "height" => {
                let height_str = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get the image height param");
                    StatusCode::BAD_REQUEST
                })?;

                height = Some(height_str.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse height as u32");
                    StatusCode::BAD_REQUEST
                })?);
            }
            _ => {
                info!("Unexpected multipart form field");
            }
        }
    }

    // Ensure we have all required fields
    let data = image_data.ok_or_else(|| {
        warn!("No 'image' field found in multipart data");
        StatusCode::BAD_REQUEST
    })?;

    let width = width.unwrap_or(800);
    let height = height.unwrap_or(600);

    debug!(
        width = width,
        height = height,
        "Using dimensions for resize"
    );

    let image_to_resize = ResizeImageInput::new(data, width, height);
    let resized_image = resize_image(image_to_resize).await?;

    Ok((
        [(header::CONTENT_TYPE, resized_image.image_mime_type)],
        resized_image.data,
    ))
}
