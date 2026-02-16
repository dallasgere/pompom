```rust
// This is an example of how to use rayon for this if I wanted parallel processing of images.
// But tokio spawn_blocking is fine for what I want right now, may implement batch endpoints
// later.
pub async fn process_and_resize(data: Bytes) -> Result<Vec<u8>, StatusCode> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let result = (|| {
            let img = load_from_memory(&data).map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?;

            let resized = img.resize(800, 600, image::imageops::FilterType::Lanczos3);

            let mut buf = std::io::Cursor::new(Vec::new());
            resized
                .write_to(&mut buf, image::ImageFormat::Png)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(buf.into_inner())
        })();
        let _ = tx.send(result);
    });

    rx.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
}
```