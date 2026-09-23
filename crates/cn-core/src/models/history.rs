use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// A single entry in the operation history log.
/// Persisted to `~/.local/state/cn-file-organizer/history.jsonl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source_dir: PathBuf,
    pub destination_dir: PathBuf,
    pub mode: String,
    pub file_count: u64,
    pub total_bytes: u64,
    pub status: OperationStatus,
    /// When the operation was undone (if it was).
    pub undone_at: Option<DateTime<Utc>>,
}

impl HistoryEntry {
    pub fn is_undoable(&self) -> bool {
        self.undone_at.is_none()
            && matches!(
                self.status,
                OperationStatus::Completed | OperationStatus::PartialSuccess
            )
    }
}

/// The lifecycle status of a history entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    /// All files moved successfully.
    Completed,
    /// Some files moved, some failed or were cancelled.
    PartialSuccess,
    /// All operations failed.
    Failed,
    /// User cancelled before completion.
    Cancelled,
    /// Operation was undone.
    Undone,
    /// Operation was partially undone.
    PartiallyUndone,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(status: OperationStatus) -> HistoryEntry {
        HistoryEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source_dir: PathBuf::from("/home/user/Downloads"),
            destination_dir: PathBuf::from("/home/user/Downloads"),
            mode: "smart".into(),
            file_count: 247,
            total_bytes: 9_055_485_952,
            status,
            undone_at: None,
        }
    }

    #[test]
    fn completed_entry_is_undoable() {
        assert!(make_entry(OperationStatus::Completed).is_undoable());
    }

    #[test]
    fn partial_success_entry_is_undoable() {
        assert!(make_entry(OperationStatus::PartialSuccess).is_undoable());
    }

    #[test]
    fn failed_entry_is_not_undoable() {
        assert!(!make_entry(OperationStatus::Failed).is_undoable());
    }

    #[test]
    fn already_undone_entry_is_not_undoable() {
        let mut e = make_entry(OperationStatus::Undone);
        e.undone_at = Some(Utc::now());
        assert!(!e.is_undoable());
    }
}
