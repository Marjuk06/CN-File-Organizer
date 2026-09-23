use serde::{Deserialize, Serialize};
use std::fmt;

/// The top-level category a file belongs to.
///
/// Classification uses magic bytes first, then extension fallback.
/// The `Other` variant catches everything that doesn't match known types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// PDF, DOCX, ODT, TXT, MD, RTF, EPUB, etc.
    Documents,
    /// PNG, JPG, WEBP, SVG, GIF, RAW, HEIC, etc.
    Images,
    /// MP4, MKV, AVI, MOV, WEBM, FLV, etc.
    Videos,
    /// MP3, FLAC, OGG, WAV, AAC, AIFF, etc.
    Audio,
    /// ZIP, TAR, GZ, BZ2, 7Z, RAR, XZ, DEB, RPM, etc.
    Archives,
    /// RS, PY, JS, TS, HTML, CSS, Java, C, CPP, Go, etc.
    Code,
    /// JSON, CSV, XML, YAML, TOML, SQLITE, DB, etc.
    Data,
    /// AppImage, ELF executables (non-archive), etc.
    Applications,
    /// TTF, OTF, WOFF, WOFF2
    Fonts,
    /// Anything that does not match the above categories
    Other,
}

impl Category {
    /// Human-readable display name (used in UI and CLI output).
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Documents => "Documents",
            Category::Images => "Images",
            Category::Videos => "Videos",
            Category::Audio => "Audio",
            Category::Archives => "Archives",
            Category::Code => "Code",
            Category::Data => "Data",
            Category::Applications => "Applications",
            Category::Fonts => "Fonts",
            Category::Other => "Other",
        }
    }

    /// Default subfolder name used during organization.
    pub fn folder_name(&self) -> &'static str {
        match self {
            Category::Documents => "Documents",
            Category::Images => "Images",
            Category::Videos => "Videos",
            Category::Audio => "Audio",
            Category::Archives => "Archives",
            Category::Code => "Code",
            Category::Data => "Data",
            Category::Applications => "Applications",
            Category::Fonts => "Fonts",
            Category::Other => "Other",
        }
    }

    /// All variants, in display order.
    pub fn all() -> &'static [Category] {
        &[
            Category::Documents,
            Category::Images,
            Category::Videos,
            Category::Audio,
            Category::Archives,
            Category::Code,
            Category::Data,
            Category::Applications,
            Category::Fonts,
            Category::Other,
        ]
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_categories_have_display_name() {
        for cat in Category::all() {
            assert!(!cat.display_name().is_empty());
        }
    }

    #[test]
    fn all_categories_have_folder_name() {
        for cat in Category::all() {
            assert!(!cat.folder_name().is_empty());
        }
    }

    #[test]
    fn category_serializes_to_snake_case() {
        let json = serde_json::to_string(&Category::Documents).unwrap();
        assert_eq!(json, r#""documents""#);
    }

    #[test]
    fn category_deserializes_from_snake_case() {
        let cat: Category = serde_json::from_str(r#""images""#).unwrap();
        assert_eq!(cat, Category::Images);
    }

    #[test]
    fn all_variants_count() {
        assert_eq!(Category::all().len(), 10);
    }
}
