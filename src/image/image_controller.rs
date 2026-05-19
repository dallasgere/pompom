use axum::{
    Json, Router,
    extract::Multipart,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
};
use tracing::{debug, info, instrument, warn};

use super::image_service::{crop_image, get_image_dimensions, resize_image};
use super::image_types::{
    CropImageInput, GetImageDimensionsInput, ImageDimensionsResponse, ImageError, ResizeImageInput,
};

impl IntoResponse for ImageError {
    fn into_response(self) -> Response {
        let status = match self {
            ImageError::BadRequest => StatusCode::BAD_REQUEST,
            ImageError::UnsupportedFormat => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ImageError::EncodeFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ImageError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}

pub fn image_routes() -> Router {
    Router::new()
        .route("/resize", post(resize_image_controller))
        .route("/crop", post(crop_image_controller))
        .route("/get_image_dimensions", post(get_image_dimensions_controller))
}

#[instrument(skip(multipart))]
pub async fn resize_image_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImageError> {
    info!("Received image resize request");

    let mut image_data = None;
    let mut width = None;
    let mut height = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        warn!(error = ?e, "Failed to read multipart field");
        ImageError::BadRequest
    })? {
        let field_name = field.name().unwrap_or("<unnamed>").to_string();
        debug!(field_name = %field_name, "Found multipart field");

        match field_name.as_str() {
            "image" => {
                image_data = Some(field.bytes().await.map_err(|e| {
                    warn!(error = ?e, "Failed to read field bytes");
                    ImageError::BadRequest
                })?);
            }
            "width" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get width param");
                    ImageError::BadRequest
                })?;
                width = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse width as u32");
                    ImageError::BadRequest
                })?);
            }
            "height" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get height param");
                    ImageError::BadRequest
                })?;
                height = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse height as u32");
                    ImageError::BadRequest
                })?);
            }
            _ => {
                info!("Unexpected multipart form field");
            }
        }
    }

    let data = image_data.ok_or_else(|| {
        warn!("No 'image' field found in multipart data");
        ImageError::BadRequest
    })?;

    let width = width.unwrap_or(800);
    let height = height.unwrap_or(600);
    debug!(width, height, "Using dimensions for resize");

    let result = resize_image(ResizeImageInput { data, width, height }).await?;

    Ok(([(header::CONTENT_TYPE, result.image_mime_type)], result.data))
}

#[instrument(skip(multipart))]
pub async fn crop_image_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImageError> {
    info!("Received image crop request");

    let mut image_data = None;
    let mut x = None;
    let mut y = None;
    let mut width = None;
    let mut height = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        warn!(error = ?e, "Failed to read multipart field");
        ImageError::BadRequest
    })? {
        let field_name = field.name().unwrap_or("<unnamed>").to_string();
        debug!(field_name = %field_name, "Found multipart field");

        match field_name.as_str() {
            "image" => {
                image_data = Some(field.bytes().await.map_err(|e| {
                    warn!(error = ?e, "Failed to read field bytes");
                    ImageError::BadRequest
                })?);
            }
            "x" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get x param");
                    ImageError::BadRequest
                })?;
                x = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse x as u32");
                    ImageError::BadRequest
                })?);
            }
            "y" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get y param");
                    ImageError::BadRequest
                })?;
                y = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse y as u32");
                    ImageError::BadRequest
                })?);
            }
            "width" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get width param");
                    ImageError::BadRequest
                })?;
                width = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse width as u32");
                    ImageError::BadRequest
                })?);
            }
            "height" => {
                let text = field.text().await.map_err(|e| {
                    warn!(error = ?e, "Failed to get height param");
                    ImageError::BadRequest
                })?;
                height = Some(text.parse::<u32>().map_err(|e| {
                    warn!(error = ?e, "Failed to parse height as u32");
                    ImageError::BadRequest
                })?);
            }
            _ => {
                info!("Unexpected multipart form field");
            }
        }
    }

    let data = image_data.ok_or_else(|| {
        warn!("No 'image' field found in multipart data");
        ImageError::BadRequest
    })?;
    let x = x.ok_or_else(|| {
        warn!("No 'x' field found in multipart data");
        ImageError::BadRequest
    })?;
    let y = y.ok_or_else(|| {
        warn!("No 'y' field found in multipart data");
        ImageError::BadRequest
    })?;
    let width = width.ok_or_else(|| {
        warn!("No 'width' field found in multipart data");
        ImageError::BadRequest
    })?;
    let height = height.ok_or_else(|| {
        warn!("No 'height' field found in multipart data");
        ImageError::BadRequest
    })?;

    debug!(x, y, width, height, "Using dimensions for crop");

    let result = crop_image(CropImageInput { data, x, y, width, height }).await?;

    Ok(([(header::CONTENT_TYPE, result.image_mime_type)], result.data))
}

#[instrument(skip(multipart))]
pub async fn get_image_dimensions_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImageError> {
    let mut image_data = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        warn!(error = ?e, "Failed to read multipart field");
        ImageError::BadRequest
    })? {
        let field_name = field.name().unwrap_or("<unnamed>").to_string();

        match field_name.as_str() {
            "image" => {
                image_data = Some(field.bytes().await.map_err(|e| {
                    warn!(error = ?e, "Failed to read image bytes");
                    ImageError::BadRequest
                })?);
            }
            _ => {
                info!("Unexpected multipart form field");
            }
        }
    }

    let data = image_data.ok_or_else(|| {
        warn!("No 'image' field found in multipart data");
        ImageError::BadRequest
    })?;

    let dims = get_image_dimensions(GetImageDimensionsInput { data }).await?;

    Ok(Json(ImageDimensionsResponse {
        width: dims.width,
        height: dims.height,
    }))
}
