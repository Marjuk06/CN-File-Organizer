use crate::models::{category::Category, file_info::FileInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// The result of scanning a directory.
/// This is what the planner works from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    /// Unique ID for this scan session.
    pub id: Uuid,
    /// The directory that was scanned.
    pub source_dir: PathBuf,
    /// All discovered files (regular files only — symlinks, special files excluded from this list).
    pub files: Vec<FileInfo>,
    /// Total number of regular files found.
    pub total_files: u64,
    /// Total size of all regular files in bytes.
    pub total_bytes: u64,
    /// Breakdown by category.
    pub by_category: HashMap<String, CategoryStats>,
    /// Number of symlinks encountered.
    pub symlink_count: u64,
    /// Number of broken symlinks encountered.
    pub broken_symlink_count: u64,
    /// Number of special files encountered.
    pub special_file_count: u64,
    /// Number of hidden files skipped (when include_hidden = false).
    pub hidden_skipped: u64,
    /// Scan options that were used.
    pub options: ScanOptions,
}

/// Per-category statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryStats {
    pub count: u64,
    pub total_bytes: u64,
}

/// Options controlling scan behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanOptions {
    /// Include hidden files (those starting with '.').
    /// Default: false.
    pub include_hidden: bool,
    /// Follow symbolic links during traversal.
    /// Default: false.
    pub follow_symlinks: bool,
    /// Maximum directory depth. None = unlimited.
    pub max_depth: Option<usize>,
    /// Glob patterns for paths to exclude.
    pub exclude_patterns: Vec<String>,
}

impl ScanSummary {
    /// Returns all files belonging to the given category.
    pub fn files_in_category(&self, category: Category) -> Vec<&FileInfo> {
        self.files
            .iter()
            .filter(|f| f.category == Some(category))
            .collect()
    }

    /// Returns the human-readable size string.
    pub fn formatted_size(&self) -> String {
        format_bytes(self.total_bytes)
    }
}

/// Format bytes into a human-readable string (e.g. "8.42 GB").
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_bytes() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn format_bytes_kilobytes() {
        assert_eq!(format_bytes(2048), "2.00 KB");
    }

    #[test]
    fn format_bytes_gigabytes() {
        assert_eq!(format_bytes(9_055_485_952), "8.43 GB");
    }

    #[test]
    fn scan_options_default_is_conservative() {
        let opts = ScanOptions::default();
        assert!(!opts.include_hidden);
        assert!(!opts.follow_symlinks);
        assert!(opts.max_depth.is_none());
    }
}
