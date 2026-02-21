use axum::body::Bytes;
use serde::Serialize;

pub struct ProcessedImageOutput {
    pub data: Vec<u8>,
    pub image_mime_type: &'static str,
}

impl ProcessedImageOutput {
    pub fn new(data: Vec<u8>, image_mime_type: &'static str) -> Self {
        Self {
            data,
            image_mime_type,
        }
    }
}

pub type ResizeImageOutput = ProcessedImageOutput;
pub type CropImageOutput = ProcessedImageOutput;

// --- Input types ---

pub struct ResizeImageInput {
    pub data: Bytes,
    pub width: u32,
    pub height: u32,
}

impl ResizeImageInput {
    pub fn new(data: Bytes, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

pub struct CropImageInput {
    pub data: Bytes,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CropImageInput {
    pub fn new(data: Bytes, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            data,
            x,
            y,
            width,
            height,
        }
    }
}

pub struct GetImageDimensionsInput {
    pub data: Bytes,
}

impl GetImageDimensionsInput {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }
}

pub struct GetImageDimensionsOutput {
    pub height: u32,
    pub width: u32,
    pub image_mime_type: &'static str,
}

impl GetImageDimensionsOutput {
    pub fn new(height: u32, width: u32, image_mime_type: &'static str) -> Self {
        Self {
            height,
            width,
            image_mime_type,
        }
    }
}

#[derive(Serialize)]
pub struct ImageDimensionsResponse {
    pub width: u32,
    pub height: u32,
}
