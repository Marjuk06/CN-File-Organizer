use thiserror::Error;

/// The unified error type for all cn-core operations.
/// Consumers can match on specific variants for fine-grained handling.
#[derive(Debug, Error)]
pub enum CnError {
    // ── Filesystem errors ────────────────────────────────────────────────────
    #[error("Source path not found: {0}")]
    SourceNotFound(String),

    #[error("Destination path not found: {0}")]
    DestinationNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Insufficient disk space: need {needed} bytes, have {available} bytes")]
    InsufficientSpace { needed: u64, available: u64 },

    #[error("Source and destination paths overlap: {0}")]
    PathsOverlap(String),

    #[error("Destination is inside source directory: {0}")]
    SelfContained(String),

    #[error("Path is a protected system directory: {0}")]
    ProtectedPath(String),

    #[error("Special file (device/FIFO/socket) cannot be organized: {0}")]
    SpecialFile(String),

    #[error("Symlink loop detected: {0}")]
    SymlinkLoop(String),

    #[error("Broken symlink: {0}")]
    BrokenSymlink(String),

    #[error("File was modified since scan: {0}")]
    FileModifiedSinceScan(String),

    #[error("File disappeared between scan and execute: {0}")]
    FileDisappeared(String),

    // ── Operation errors ─────────────────────────────────────────────────────
    #[error("Operation not found: {0}")]
    OperationNotFound(String),

    #[error("Operation cannot be undone: {0}")]
    NotUndoable(String),

    #[error("Scan not found: {0}")]
    ScanNotFound(String),

    #[error("Plan not found: {0}")]
    PlanNotFound(String),

    #[error("Operation was cancelled")]
    Cancelled,

    #[error("Cross-filesystem move verification failed: SHA-256 mismatch for {0}")]
    VerificationFailed(String),

    // ── Rule errors ──────────────────────────────────────────────────────────
    #[error("Invalid rule: {0}")]
    InvalidRule(String),

    #[error("Rule not found: {0}")]
    RuleNotFound(String),

    // ── Config errors ────────────────────────────────────────────────────────
    #[error("Config error: {0}")]
    Config(String),

    #[error("Config schema version {found} is newer than supported version {supported}")]
    ConfigVersionTooNew { found: u32, supported: u32 },

    // ── IO / system errors ───────────────────────────────────────────────────
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Journal error: {0}")]
    Journal(String),

    // ── Catch-all ────────────────────────────────────────────────────────────
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for CnError {
    fn from(e: serde_json::Error) -> Self {
        CnError::Serialization(e.to_string())
    }
}

impl From<toml::ser::Error> for CnError {
    fn from(e: toml::ser::Error) -> Self {
        CnError::Serialization(e.to_string())
    }
}

impl From<toml::de::Error> for CnError {
    fn from(e: toml::de::Error) -> Self {
        CnError::Serialization(e.to_string())
    }
}

/// Convenience result type for all cn-core functions.
pub type CnResult<T> = Result<T, CnError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_source_not_found() {
        let e = CnError::SourceNotFound("/tmp/missing".to_string());
        assert!(e.to_string().contains("/tmp/missing"));
    }

    #[test]
    fn error_display_insufficient_space() {
        let e = CnError::InsufficientSpace {
            needed: 1_000_000,
            available: 500_000,
        };
        assert!(e.to_string().contains("1000000"));
        assert!(e.to_string().contains("500000"));
    }

    #[test]
    fn io_error_converts() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let cn_err: CnError = io_err.into();
        assert!(matches!(cn_err, CnError::Io(_)));
    }
}
