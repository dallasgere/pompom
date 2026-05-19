use super::image_types::{
    CropImageInput, GetImageDimensionsInput, GetImageDimensionsOutput, ImageError,
    ProcessedImageOutput, ResizeImageInput,
};
use image::{GenericImageView, guess_format, load_from_memory};
use tracing::{debug, error, instrument, warn};

#[instrument(skip(input), fields(data_size = input.data.len(), target_width = input.width, target_height = input.height))]
pub async fn resize_image(input: ResizeImageInput) -> Result<ProcessedImageOutput, ImageError> {
    debug!("Spawning blocking task for image processing");

    tokio::task::spawn_blocking(move || {
        let image_format = guess_format(&input.data).map_err(|e| {
            error!(error = %e, "Failed to guess image format");
            ImageError::UnsupportedFormat
        })?;

        debug!("Loading image from memory");
        let img = load_from_memory(&input.data).map_err(|e| {
            error!(error = %e, "Failed to load image from memory");
            ImageError::UnsupportedFormat
        })?;

        let (original_width, original_height) = img.dimensions();
        debug!(width = original_width, height = original_height, "Loaded image, starting resize");

        let resized = img.resize(input.width, input.height, image::imageops::FilterType::Lanczos3);
        debug!(target_width = input.width, target_height = input.height, "Image resized");

        let mut buf = std::io::Cursor::new(Vec::new());
        resized.write_to(&mut buf, image_format).map_err(|e| {
            error!(error = %e, "Failed to encode image");
            ImageError::EncodeFailed
        })?;

        debug!(output_bytes = buf.get_ref().len(), "Image encoded successfully");

        Ok(ProcessedImageOutput {
            data: buf.into_inner(),
            image_mime_type: image_format.to_mime_type(),
        })
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        ImageError::InternalError
    })?
}

#[instrument(skip(input), fields(data_size = input.data.len()))]
pub async fn crop_image(input: CropImageInput) -> Result<ProcessedImageOutput, ImageError> {
    tokio::task::spawn_blocking(move || {
        let image_format = guess_format(&input.data).map_err(|e| {
            error!(error = %e, "Failed to guess image format");
            ImageError::UnsupportedFormat
        })?;

        let mut img = load_from_memory(&input.data).map_err(|e| {
            error!(error = %e, "Failed to load image from memory");
            ImageError::UnsupportedFormat
        })?;

        let cropped = img.crop(input.x, input.y, input.width, input.height);

        let mut buf = std::io::Cursor::new(Vec::new());
        cropped.write_to(&mut buf, image_format).map_err(|e| {
            error!(error = %e, "Failed to encode image");
            ImageError::EncodeFailed
        })?;

        Ok(ProcessedImageOutput {
            data: buf.into_inner(),
            image_mime_type: image_format.to_mime_type(),
        })
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        ImageError::InternalError
    })?
}

#[instrument(skip(input), fields(data_size = input.data.len()))]
pub async fn get_image_dimensions(
    input: GetImageDimensionsInput,
) -> Result<GetImageDimensionsOutput, ImageError> {
    tokio::task::spawn_blocking(move || {
        let image_format = guess_format(&input.data).map_err(|e| {
            error!(error = %e, "Failed to guess image format");
            ImageError::UnsupportedFormat
        })?;

        let img = load_from_memory(&input.data).map_err(|e| {
            error!(error = %e, "Failed to load image from memory");
            ImageError::UnsupportedFormat
        })?;

        Ok(GetImageDimensionsOutput {
            height: img.height(),
            width: img.width(),
            image_mime_type: image_format.to_mime_type(),
        })
    })
    .await
    .map_err(|e| {
        warn!(error = ?e, "Blocking task panicked");
        ImageError::InternalError
    })?
}
