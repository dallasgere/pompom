use axum::body::Bytes;
use serde::Serialize;

pub enum ImageError {
    BadRequest,
    UnsupportedFormat,
    EncodeFailed,
    InternalError,
}

pub struct ProcessedImageOutput {
    pub data: Vec<u8>,
    pub image_mime_type: &'static str,
}

pub struct ResizeImageInput {
    pub data: Bytes,
    pub width: u32,
    pub height: u32,
}

pub struct CropImageInput {
    pub data: Bytes,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct GetImageDimensionsInput {
    pub data: Bytes,
}

pub struct GetImageDimensionsOutput {
    pub height: u32,
    pub width: u32,
    pub image_mime_type: &'static str,
}

#[derive(Serialize)]
pub struct ImageDimensionsResponse {
    pub width: u32,
    pub height: u32,
}
