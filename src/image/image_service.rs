use super::image_types::{ResizeImageInput, ResizeImageOutput};
use axum::body::Bytes;
use axum::http::StatusCode;
use image::{GenericImageView, guess_format, load_from_memory};
use tracing::{debug, error, instrument, warn};

/// Resize an image from bytes
#[instrument(skip(image_to_resize), fields(data_size = image_to_resize.data.len(), target_width = image_to_resize.width, target_height = image_to_resize.height))]
pub async fn resize_image(
    image_to_resize: ResizeImageInput,
) -> Result<ResizeImageOutput, StatusCode> {
    debug!("Spawning blocking task for image processing");

    tokio::task::spawn_blocking(move || {
        let image_format = guess_format(&image_to_resize.data).map_err(|e| {
            error!(error = %e, "Failed to guess image format when resizing image");
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        })?;

        debug!("Loading image from memory");
        let img = load_from_memory(&image_to_resize.data).map_err(|e| {
            error!(error = %e, "Failed to load image from memory");
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        })?;

        let (original_width, original_height) = img.dimensions();
        debug!(
            width = original_width,
            height = original_height,
            "Loaded image, starting resize"
        );

        let resized = img.resize(
            image_to_resize.width,
            image_to_resize.height,
            image::imageops::FilterType::Lanczos3,
        );
        debug!(
            target_width = image_to_resize.width,
            target_height = image_to_resize.height,
            "Image resized"
        );

        let mut buf = std::io::Cursor::new(Vec::new());
        resized.write_to(&mut buf, image_format).map_err(|e| {
            error!(error = %e, "Failed to encode image");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let output_size = buf.get_ref().len();
        debug!(output_bytes = output_size, "Image encoded successfully");

        let result = ResizeImageOutput::new(buf.into_inner(), image_format.to_mime_type());

        Ok(result)
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
}

pub async fn crop_image(data: Bytes) -> Result<Bytes, StatusCode> {
    tokio::task::spawn_blocking(move || {
        let image_format = guess_format(&data).map_err(|e| {
            error!(error = %e, "Failed to guess image format when resizing image");
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        })?;

        Ok(data)
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
}
