//! Font loading, caching, and management.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use super::FontMetrics;

/// Font weight (100-900).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontWeight(pub u16);

impl FontWeight {
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const BLACK: Self = Self(900);
}

/// Font style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font family descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontFamily {
    pub name: String,
}

/// A loaded font with its metrics and binary data.
#[derive(Debug, Clone)]
pub struct Font {
    pub family: FontFamily,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub metrics: FontMetrics,
    pub data: Arc<Vec<u8>>,
}

impl Font {
    /// Load a font from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, FontError> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data)
    }

    /// Load a font from raw bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, FontError> {
        // Parse font using ttf-parser
        let face = ttf_parser::Face::parse(&data, 0)
            .map_err(|_| FontError::InvalidData)?;

        // Extract font metadata
        let family = FontFamily {
            name: face
                .names()
                .into_iter()
                .find(|n| n.name_id == ttf_parser::name_id::FULL_NAME && n.is_unicode())
                .and_then(|n| n.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
        };

        let weight = FontWeight(face.weight().to_number());
        
        let style = if face.is_italic() {
            FontStyle::Italic
        } else if face.is_oblique() {
            FontStyle::Oblique
        } else {
            FontStyle::Normal
        };

        // Extract font metrics
        let units_per_em = face.units_per_em();
        let ascent = face.ascender();
        let descent = face.descender();
        let line_gap = face.line_gap();

        let metrics = FontMetrics {
            ascent: (ascent as f32).into(),
            descent: (descent as f32).into(),
            line_gap: (line_gap as f32).into(),
            units_per_em,
        };

        Ok(Self {
            family,
            weight,
            style,
            metrics,
            data: Arc::new(data),
        })
    }

    /// Get the font's metrics.
    #[inline]
    pub fn metrics(&self) -> FontMetrics {
        self.metrics
    }
}

/// Font cache for efficient font reuse.
pub struct FontCache {
    fonts: RwLock<HashMap<FontKey, Arc<Font>>>,
    system_fonts: RwLock<Vec<Arc<Font>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontKey {
    family: FontFamily,
    weight: FontWeight,
    style: FontStyle,
}

impl FontCache {
    /// Create a new empty font cache.
    pub fn new() -> Self {
        Self {
            fonts: RwLock::new(HashMap::new()),
            system_fonts: RwLock::new(Vec::new()),
        }
    }

    /// Load system fonts (platform-specific).
    pub fn load_system_fonts(&self) -> Result<(), FontError> {
        // Use fontdb for cross-platform system font discovery
        let mut db = fontdb::Database::new();
        db.load_system_fonts();

        let mut system_fonts = self.system_fonts.write().unwrap();
        
        for face_info in db.faces() {
            // Try to load the font file
            if let fontdb::Source::File(ref path) = face_info.source {
                if let Ok(font) = Font::from_path(path) {
                    system_fonts.push(Arc::new(font));
                }
            }
            // Note: Binary sources are typically embedded fonts, skip for now
        }

        Ok(())
    }

    /// Get or load a font matching the descriptor.
    pub fn get_font(
        &self,
        family: &FontFamily,
        weight: FontWeight,
        style: FontStyle,
    ) -> Option<Arc<Font>> {
        let key = FontKey {
            family: family.clone(),
            weight,
            style,
        };

        // Fast path: already cached
        if let Some(font) = self.fonts.read().unwrap().get(&key) {
            return Some(Arc::clone(font));
        }

        // Slow path: search system fonts
        let system_fonts = self.system_fonts.read().unwrap();
        for font in system_fonts.iter() {
            if font.family == *family && font.weight == weight && font.style == style {
                let font = Arc::clone(font);
                self.fonts.write().unwrap().insert(key, Arc::clone(&font));
                return Some(font);
            }
        }

        None
    }

    /// Add a font to the cache.
    pub fn add_font(&self, font: Font) {
        let key = FontKey {
            family: font.family.clone(),
            weight: font.weight,
            style: font.style,
        };
        let font = Arc::new(font);
        self.fonts.write().unwrap().insert(key, font.clone());
        self.system_fonts.write().unwrap().push(font);
    }
}

impl Default for FontCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Font-related errors.
#[derive(Debug, thiserror::Error)]
pub enum FontError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid font data")]
    InvalidData,

    #[error("font not found: {0}")]
    NotFound(String),
}
