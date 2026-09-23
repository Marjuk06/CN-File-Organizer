use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// The result of executing an operation plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub operation_id: Uuid,
    /// Total files that were moved/copied successfully.
    pub succeeded: u64,
    /// Files that were skipped (due to conflict strategy or filters).
    pub skipped: u64,
    /// Files that failed to move/copy.
    pub failed: u64,
    /// Individual file results.
    pub file_results: Vec<FileResult>,
    /// Overall status.
    pub status: ExecutionStatus,
    /// Duration in milliseconds.
    pub duration_ms: u64,
}

impl ExecutionResult {
    pub fn total_files(&self) -> u64 {
        self.succeeded + self.skipped + self.failed
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
            && matches!(
                self.status,
                ExecutionStatus::Completed | ExecutionStatus::PartialSuccess
            )
    }
}

/// The result for a single file operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub operation_id: Uuid,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub outcome: FileOutcome,
    /// Error message if outcome is Failed.
    pub error: Option<String>,
    /// SHA-256 of destination file (set for cross-fs moves after verification).
    pub verified_hash: Option<String>,
    /// Timestamp of destination file after move (used for undo safety check).
    pub destination_modified_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// The outcome of a single file operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileOutcome {
    /// File was successfully moved or copied.
    Moved,
    /// File was skipped (conflict strategy or filter).
    Skipped,
    /// File failed to move/copy.
    Failed,
    /// File was renamed to avoid conflict.
    Renamed,
}

/// The overall status of an execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    /// All operations completed successfully.
    Completed,
    /// Some operations succeeded, some failed or were cancelled.
    PartialSuccess,
    /// All operations failed.
    Failed,
    /// Operation was cancelled by the user.
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(succeeded: u64, failed: u64, status: ExecutionStatus) -> ExecutionResult {
        ExecutionResult {
            operation_id: Uuid::new_v4(),
            succeeded,
            skipped: 0,
            failed,
            file_results: vec![],
            status,
            duration_ms: 100,
        }
    }

    #[test]
    fn completed_with_no_failures_is_success() {
        let r = make_result(10, 0, ExecutionStatus::Completed);
        assert!(r.is_success());
    }

    #[test]
    fn partial_with_failures_is_not_success() {
        let r = make_result(8, 2, ExecutionStatus::PartialSuccess);
        assert!(!r.is_success());
    }

    #[test]
    fn total_files_sums_correctly() {
        let mut r = make_result(10, 2, ExecutionStatus::PartialSuccess);
        r.skipped = 3;
        assert_eq!(r.total_files(), 15);
    }
}
