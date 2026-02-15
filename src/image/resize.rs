use axum::body::Bytes;
use axum::http::StatusCode;
use image::{load_from_memory, GenericImageView};
use tracing::{debug, error, instrument, warn};

/// Resize an image from bytes
#[instrument(skip(data), fields(data_size = data.len()))]
pub async fn resize_image(data: Bytes) -> Result<Vec<u8>, StatusCode> {
    debug!("Spawning blocking task for image processing");

    tokio::task::spawn_blocking(move || {
        debug!("Loading image from memory");
        let img = load_from_memory(&data).map_err(|e| {
            error!(error = %e, "Failed to load image from memory");
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        })?;

        let (original_width, original_height) = img.dimensions();
        debug!(
            width = original_width,
            height = original_height,
            "Loaded image, starting resize"
        );

        let resized = img.resize(800, 600, image::imageops::FilterType::Lanczos3);
        debug!("Image resized to 800x600");

        let mut buf = std::io::Cursor::new(Vec::new());
        let _ = resized
            .write_to(&mut buf, image::ImageFormat::Jpeg)
            .map_err(|e| {
                error!(error = %e, "Failed to encode image to JPEG");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let output_size = buf.get_ref().len();
        debug!(output_bytes = output_size, "Image encoded successfully");

        Ok(buf.into_inner())
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
}

// This is an example of how to use rayon for this if I wanted parallel processing of images.
// But tokio spawn_blocking is fine for what I want right now, may implement batch endpoints
// later.
// pub async fn process_and_resize(data: Bytes) -> Result<Vec<u8>, StatusCode> {
//     let (tx, rx) = tokio::sync::oneshot::channel();
//
//     rayon::spawn(move || {
//         let result = (|| {
//             let img = load_from_memory(&data).map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?;
//
//             let resized = img.resize(800, 600, image::imageops::FilterType::Lanczos3);
//
//             let mut buf = std::io::Cursor::new(Vec::new());
//             resized
//                 .write_to(&mut buf, image::ImageFormat::Png)
//                 .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//
//             Ok(buf.into_inner())
//         })();
//         let _ = tx.send(result);
//     });
//
//     rx.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
// }
