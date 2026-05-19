use super::image_types::ImageError;
use axum::{
    body::Bytes,
    extract::multipart::Field,
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for ImageError {
    fn into_response(self) -> Response {
        let status = match self {
            ImageError::BadRequest => StatusCode::BAD_REQUEST,
            ImageError::UnsupportedFormat => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ImageError::EncodeFailed | ImageError::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status.into_response()
    }
}

pub async fn read_bytes(field: Field<'_>) -> Result<Bytes, ImageError> {
    field.bytes().await.map_err(|_| ImageError::BadRequest)
}

pub async fn read_u32(field: Field<'_>) -> Result<u32, ImageError> {
    field
        .text()
        .await
        .map_err(|_| ImageError::BadRequest)?
        .parse()
        .map_err(|_| ImageError::BadRequest)
}
