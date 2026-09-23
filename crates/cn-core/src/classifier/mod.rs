pub mod mime;
pub mod category;
pub mod extension;

use crate::models::category::Category;
use std::path::Path;

/// Classify a file and return its Category and optional MIME type string.
///
/// Classification priority:
/// 1. Magic bytes (infer crate) — most reliable
/// 2. File extension (built-in lookup table) — fallback
/// 3. Category::Other — if nothing matches
pub fn classify(path: &Path) -> (Category, Option<String>) {
    // Try magic bytes first (reads up to 16 bytes from the file)
    if let Some((cat, mime)) = mime::detect_by_magic(path) {
        return (cat, Some(mime));
    }

    // Fall back to extension
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if let Some(cat) = extension::category_for_extension(&ext.to_lowercase()) {
            return (cat, None);
        }
    }

    (Category::Other, None)
}
