use crate::models::category::Category;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a single file discovered during a scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// File size in bytes.
    pub size: u64,
    /// Detected file kind.
    pub kind: FileKind,
    /// Detected category (only set for regular files).
    pub category: Option<Category>,
    /// MIME type string if detected (e.g. "image/png").
    pub mime_type: Option<String>,
    /// File extension (lowercase, without leading dot). None if no extension.
    pub extension: Option<String>,
    /// Last modified time.
    pub modified_at: Option<DateTime<Utc>>,
    /// Created time (not reliable on all Linux filesystems).
    pub created_at: Option<DateTime<Utc>>,
    /// Whether the file is hidden (starts with '.').
    pub is_hidden: bool,
}

/// The type of filesystem object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileKind {
    /// A regular file.
    Regular,
    /// A symbolic link whose target exists.
    Symlink,
    /// A symbolic link whose target does not exist.
    BrokenSymlink,
    /// A device file, FIFO, socket, or other special object.
    Special,
    /// A directory (included when scanning yields nested items).
    Directory,
}

impl FileInfo {
    /// Returns true if this file can be organized (regular files only).
    pub fn is_organizable(&self) -> bool {
        self.kind == FileKind::Regular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_file_is_organizable() {
        let f = FileInfo {
            path: PathBuf::from("/tmp/test.pdf"),
            size: 1024,
            kind: FileKind::Regular,
            category: Some(Category::Documents),
            mime_type: Some("application/pdf".into()),
            extension: Some("pdf".into()),
            modified_at: None,
            created_at: None,
            is_hidden: false,
        };
        assert!(f.is_organizable());
    }

    #[test]
    fn symlink_is_not_organizable() {
        let f = FileInfo {
            path: PathBuf::from("/tmp/link"),
            size: 0,
            kind: FileKind::Symlink,
            category: None,
            mime_type: None,
            extension: None,
            modified_at: None,
            created_at: None,
            is_hidden: false,
        };
        assert!(!f.is_organizable());
    }

    #[test]
    fn file_kind_serializes() {
        let json = serde_json::to_string(&FileKind::BrokenSymlink).unwrap();
        assert_eq!(json, r#""broken_symlink""#);
    }
}
