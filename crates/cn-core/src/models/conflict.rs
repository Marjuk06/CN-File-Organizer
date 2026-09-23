use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about a conflict between an incoming file and an existing destination file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub existing_path: PathBuf,
    pub existing_size: u64,
    pub existing_modified: Option<DateTime<Utc>>,
    pub incoming_path: PathBuf,
    pub incoming_size: u64,
    pub incoming_modified: Option<DateTime<Utc>>,
    /// SHA-256 of the existing file, computed only on demand.
    pub existing_hash: Option<String>,
    /// SHA-256 of the incoming file, computed only on demand.
    pub incoming_hash: Option<String>,
}

impl ConflictInfo {
    /// Returns true if both files appear to be identical based on size.
    /// A full hash comparison gives certainty.
    pub fn sizes_match(&self) -> bool {
        self.existing_size == self.incoming_size
    }

    /// Returns true if hashes are available and identical.
    pub fn confirmed_duplicate(&self) -> bool {
        match (&self.existing_hash, &self.incoming_hash) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Generate a renamed destination path that doesn't conflict.
    /// e.g. "photo.jpg" → "photo (1).jpg", "photo (1).jpg" → "photo (2).jpg"
    pub fn generate_rename(path: &std::path::Path) -> PathBuf {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let ext = path.extension().and_then(|s| s.to_str());
        let parent = path.parent().unwrap_or(std::path::Path::new(""));

        // Try "(1)", "(2)", ... up to 9999
        for n in 1..=9999u32 {
            let new_name = if let Some(ext) = ext {
                format!("{} ({}).{}", stem, n, ext)
            } else {
                format!("{} ({})", stem, n)
            };
            let candidate = parent.join(&new_name);
            if !candidate.exists() {
                return candidate;
            }
        }

        // Fallback with UUID suffix (should never reach here)
        let uuid = uuid::Uuid::new_v4();
        let new_name = if let Some(ext) = ext {
            format!("{}_{}.{}", stem, &uuid.to_string()[..8], ext)
        } else {
            format!("{}_{}", stem, &uuid.to_string()[..8])
        };
        parent.join(new_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn sizes_match_true() {
        let c = make_conflict(100, 100);
        assert!(c.sizes_match());
    }

    #[test]
    fn sizes_match_false() {
        let c = make_conflict(100, 200);
        assert!(!c.sizes_match());
    }

    #[test]
    fn confirmed_duplicate_with_matching_hashes() {
        let mut c = make_conflict(100, 100);
        let hash = "abc123".to_string();
        c.existing_hash = Some(hash.clone());
        c.incoming_hash = Some(hash);
        assert!(c.confirmed_duplicate());
    }

    #[test]
    fn confirmed_duplicate_with_mismatched_hashes() {
        let mut c = make_conflict(100, 100);
        c.existing_hash = Some("abc".into());
        c.incoming_hash = Some("xyz".into());
        assert!(!c.confirmed_duplicate());
    }

    #[test]
    fn generate_rename_adds_number_suffix() {
        // In a temp dir, the file doesn't exist, so "(1)" should be returned immediately
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("photo.jpg");
        let renamed = ConflictInfo::generate_rename(&src);
        let name = renamed.file_name().unwrap().to_str().unwrap();
        assert_eq!(name, "photo (1).jpg");
    }

    #[test]
    fn generate_rename_no_extension() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("README");
        let renamed = ConflictInfo::generate_rename(&src);
        let name = renamed.file_name().unwrap().to_str().unwrap();
        assert_eq!(name, "README (1)");
    }

    fn make_conflict(existing_size: u64, incoming_size: u64) -> ConflictInfo {
        ConflictInfo {
            existing_path: PathBuf::from("/dst/file.txt"),
            existing_size,
            existing_modified: None,
            incoming_path: PathBuf::from("/src/file.txt"),
            incoming_size,
            incoming_modified: None,
            existing_hash: None,
            incoming_hash: None,
        }
    }
}
