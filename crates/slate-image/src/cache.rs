//! Image caching system.

use super::DecodedImage;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Image cache for efficient reuse.
pub struct ImageCache {
    cache: Arc<RwLock<HashMap<String, Arc<DecodedImage>>>>,
    max_size: usize,
    current_size: Arc<RwLock<usize>>,
}

impl ImageCache {
    /// Create a new image cache with max size in bytes.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            current_size: Arc::new(RwLock::new(0)),
        }
    }

    /// Get an image from cache.
    pub fn get(&self, url: &str) -> Option<Arc<DecodedImage>> {
        self.cache.read().unwrap().get(url).cloned()
    }

    /// Put an image into cache.
    pub fn put(&self, url: String, image: DecodedImage) {
        let image_size = image.pixels.len();

        // Check if we need to evict
        {
            let current_size = self.current_size.read().unwrap();
            if *current_size + image_size > self.max_size {
                drop(current_size); // Release read lock before evicting
                self.evict_lru();
            }
        }

        // Add to cache
        let image = Arc::new(image);
        self.cache.write().unwrap().insert(url, image);
        
        let mut current_size = self.current_size.write().unwrap();
        *current_size += image_size;
    }

    /// Evict least recently used images.
    fn evict_lru(&self) {
        // Simple eviction: remove first entry
        // In real implementation, use LRU algorithm
        let mut cache = self.cache.write().unwrap();
        
        // Get the first key to remove
        let key_to_remove = cache.keys().next().cloned();
        
        if let Some(key) = key_to_remove {
            if let Some(image) = cache.remove(&key) {
                drop(cache); // Release cache lock before acquiring size lock
                
                let mut current_size = self.current_size.write().unwrap();
                *current_size = current_size.saturating_sub(image.pixels.len());
            }
        }
    }

    /// Clear the cache.
    pub fn clear(&self) {
        self.cache.write().unwrap().clear();
        *self.current_size.write().unwrap() = 0;
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.read().unwrap();
        let current_size = *self.current_size.read().unwrap();

        CacheStats {
            entries: cache.len(),
            size_bytes: current_size,
            max_size_bytes: self.max_size,
        }
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        // Default: 100MB cache
        Self::new(100 * 1024 * 1024)
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub size_bytes: usize,
    pub max_size_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ImageFormat;

    #[test]
    fn cache_operations() {
        let cache = ImageCache::new(1024 * 1024); // 1MB

        let pixels = vec![255u8; 100 * 100 * 4];
        let image = DecodedImage::new(100, 100, pixels, ImageFormat::Png);

        cache.put("test.png".to_string(), image);

        let retrieved = cache.get("test.png");
        assert!(retrieved.is_some());

        let stats = cache.stats();
        assert_eq!(stats.entries, 1);
    }

    #[test]
    fn cache_eviction() {
        let cache = ImageCache::new(100 * 100 * 4 * 2); // Space for 2 images

        for i in 0..3 {
            let pixels = vec![255u8; 100 * 100 * 4];
            let image = DecodedImage::new(100, 100, pixels, ImageFormat::Png);
            cache.put(format!("test{}.png", i), image);
        }

        let stats = cache.stats();
        assert!(stats.entries <= 2);
    }
}
