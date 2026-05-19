use axum::{
    Json, Router,
    body::Bytes,
    extract::{Multipart, multipart::Field},
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
            ImageError::EncodeFailed | ImageError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}

async fn read_bytes(field: Field<'_>) -> Result<Bytes, ImageError> {
    field.bytes().await.map_err(|_| ImageError::BadRequest)
}

async fn read_u32(field: Field<'_>) -> Result<u32, ImageError> {
    field
        .text()
        .await
        .map_err(|_| ImageError::BadRequest)?
        .parse()
        .map_err(|_| ImageError::BadRequest)
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

    while let Some(field) = multipart.next_field().await.map_err(|_| ImageError::BadRequest)? {
        match field.name().unwrap_or("") {
            "image" => image_data = Some(read_bytes(field).await?),
            "width" => width = Some(read_u32(field).await?),
            "height" => height = Some(read_u32(field).await?),
            _ => {}
        }
    }

    let data = image_data.ok_or_else(|| {
        warn!("Missing required 'image' field");
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

    while let Some(field) = multipart.next_field().await.map_err(|_| ImageError::BadRequest)? {
        match field.name().unwrap_or("") {
            "image" => image_data = Some(read_bytes(field).await?),
            "x" => x = Some(read_u32(field).await?),
            "y" => y = Some(read_u32(field).await?),
            "width" => width = Some(read_u32(field).await?),
            "height" => height = Some(read_u32(field).await?),
            _ => {}
        }
    }

    let data = image_data.ok_or_else(|| { warn!("Missing required 'image' field"); ImageError::BadRequest })?;
    let x = x.ok_or_else(|| { warn!("Missing required 'x' field"); ImageError::BadRequest })?;
    let y = y.ok_or_else(|| { warn!("Missing required 'y' field"); ImageError::BadRequest })?;
    let width = width.ok_or_else(|| { warn!("Missing required 'width' field"); ImageError::BadRequest })?;
    let height = height.ok_or_else(|| { warn!("Missing required 'height' field"); ImageError::BadRequest })?;
    debug!(x, y, width, height, "Using dimensions for crop");

    let result = crop_image(CropImageInput { data, x, y, width, height }).await?;

    Ok(([(header::CONTENT_TYPE, result.image_mime_type)], result.data))
}

#[instrument(skip(multipart))]
pub async fn get_image_dimensions_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ImageError> {
    let mut image_data = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| ImageError::BadRequest)? {
        if field.name().unwrap_or("") == "image" {
            image_data = Some(read_bytes(field).await?);
        }
    }

    let data = image_data.ok_or_else(|| {
        warn!("Missing required 'image' field");
        ImageError::BadRequest
    })?;

    let dims = get_image_dimensions(GetImageDimensionsInput { data }).await?;

    Ok(Json(ImageDimensionsResponse { width: dims.width, height: dims.height }))
}
