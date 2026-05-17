//! Async image loading.

use super::{DecodedImage, ImageCache, ImageDecoder, ImageError};
use std::sync::Arc;

/// Async image loader.
pub struct ImageLoader {
    cache: Arc<ImageCache>,
}

impl ImageLoader {
    /// Create a new image loader.
    pub fn new(cache: Arc<ImageCache>) -> Self {
        Self { cache }
    }

    /// Load an image from URL (async).
    pub async fn load(&self, url: &str) -> Result<Arc<DecodedImage>, ImageError> {
        // Check cache first
        if let Some(cached) = self.cache.get(url) {
            return Ok(cached);
        }

        // Load from network or file
        let image = if url.starts_with("http://") || url.starts_with("https://") {
            self.load_from_network(url).await?
        } else if url.starts_with("data:") {
            self.load_from_data_url(url)?
        } else {
            self.load_from_file(url)?
        };

        // Cache it
        self.cache.put(url.to_string(), image.clone());

        Ok(Arc::new(image))
    }

    /// Load image from network.
    async fn load_from_network(&self, _url: &str) -> Result<DecodedImage, ImageError> {
        // In real implementation, use reqwest
        // let response = reqwest::get(url).await?;
        // let bytes = response.bytes().await?;
        // ImageDecoder::decode(&bytes)

        Err(ImageError::DecodeError("Network loading not implemented".to_string()))
    }

    /// Load image from data URL.
    fn load_from_data_url(&self, url: &str) -> Result<DecodedImage, ImageError> {
        // Parse data URL: data:image/png;base64,iVBORw0KG...
        let parts: Vec<&str> = url.split(',').collect();
        if parts.len() != 2 {
            return Err(ImageError::DecodeError("Invalid data URL".to_string()));
        }

        let data = parts[1];

        // Decode base64
        let bytes = base64_decode(data)
            .map_err(|_| ImageError::DecodeError("Base64 decode failed".to_string()))?;

        ImageDecoder::decode(&bytes)
    }

    /// Load image from file.
    fn load_from_file(&self, path: &str) -> Result<DecodedImage, ImageError> {
        ImageDecoder::decode_file(path)
    }

    /// Preload multiple images.
    pub async fn preload(&self, urls: Vec<String>) -> Vec<Result<Arc<DecodedImage>, ImageError>> {
        let mut results = Vec::new();

        for url in urls {
            results.push(self.load(&url).await);
        }

        results
    }
}

/// Simple base64 decoder.
fn base64_decode(_input: &str) -> Result<Vec<u8>, ()> {
    // In real implementation, use base64 crate
    // For now, return error
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_loader() {
        let cache = Arc::new(ImageCache::default());
        let _loader = ImageLoader::new(cache);
        // Loader created successfully
    }
}
