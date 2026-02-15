use axum::{
    Router,
    extract::Multipart,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::post,
};

use super::resize::resize_image;

/// Image processing router
pub fn image_routes() -> Router {
    Router::new().route("/resize", post(resize_image_controller))
}

/// resize image endpoint
pub async fn resize_image_controller(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    eprintln!("Received multipart request");

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("<unnamed>");
        eprintln!("Found field: {}", field_name);

        if field.name() == Some("image") {
            eprintln!("Processing image field");
            let data = field.bytes().await.map_err(|e| {
                eprintln!("Failed to read field bytes: {:?}", e);
                StatusCode::BAD_REQUEST
            })?;

            eprintln!("Read {} bytes", data.len());
            let resized_bytes = resize_image(data).await?;

            return Ok((
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/jpg")],
                resized_bytes,
            ));
        }
    }

    eprintln!("No 'image' field found in multipart data");
    Err(StatusCode::BAD_REQUEST)
}
