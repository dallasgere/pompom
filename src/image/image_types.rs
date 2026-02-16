use axum::body::Bytes;

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

pub struct ResizeImageOutput {
    pub data: Vec<u8>,
    pub image_mime_type: &'static str,
}

impl ResizeImageOutput {
    pub fn new(data: Vec<u8>, image_mime_type: &'static str) -> Self {
        Self {
            data,
            image_mime_type,
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

pub struct CropImageOutput {
    pub data: Vec<u8>,
    pub image_mime_type: &'static str,
}

impl CropImageOutput {
    pub fn new(data: Vec<u8>, image_mime_type: &'static str) -> Self {
        Self {
            data,
            image_mime_type,
        }
    }
}
