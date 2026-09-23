use crate::models::{category::Category, conflict::ConflictInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// The complete plan for an organize operation.
/// Created by the planner; executed by the executor.
/// Never mutated after creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationPlan {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    /// The directory being organized.
    pub source_dir: PathBuf,
    /// Where organized files go. Equal to source_dir for in-place mode.
    pub destination_dir: PathBuf,
    /// The organize mode selected by the user.
    pub mode: OrganizeMode,
    /// Individual file operations.
    pub operations: Vec<FileOperation>,
    /// How conflicts should be handled.
    pub conflict_strategy: ConflictStrategy,
    /// Estimated total bytes to move.
    pub estimated_bytes: u64,
    /// True if source and destination are on different filesystems.
    pub cross_filesystem: bool,
    /// Overall safety evaluation.
    pub safety_status: SafetyStatus,
}

impl OperationPlan {
    pub fn total_files(&self) -> usize {
        self.operations
            .iter()
            .filter(|op| op.kind != OperationKind::Skip)
            .count()
    }
}

/// A single file move/copy/skip operation within a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub id: Uuid,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub kind: OperationKind,
    pub category: Option<Category>,
    pub size: u64,
    /// Conflict detected at planning time (if any).
    pub conflict: Option<ConflictInfo>,
}

/// What to do with a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    /// Move the file (rename on same fs; copy+verify+delete cross-fs).
    Move,
    /// Copy the file without deleting the source.
    Copy,
    /// Skip this file.
    Skip,
}

/// The organize method chosen by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum OrganizeMode {
    /// Automatically choose the best layout by category.
    Smart,
    /// Group by detected Category.
    ByCategory,
    /// Group by lowercase file extension.
    ByExtension,
    /// Group by modification date.
    ByDate { granularity: DateGranularity },
    /// Group by file size ranges.
    BySize { thresholds: SizeThresholds },
    /// Apply user-defined custom rules.
    CustomRules,
}

/// How finely to group files by date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateGranularity {
    /// e.g. "2026"
    Year,
    /// e.g. "2026-09"
    Month,
    /// e.g. "2026-09-23"
    Day,
}

/// Size threshold configuration for BySize mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeThresholds {
    /// Upper bound for "Small" (bytes). e.g. 1 MB
    pub small: u64,
    /// Upper bound for "Medium" (bytes). e.g. 100 MB
    pub medium: u64,
    /// Upper bound for "Large" (bytes). e.g. 1 GB
    pub large: u64,
    // Files above large are "Huge"
}

impl Default for SizeThresholds {
    fn default() -> Self {
        SizeThresholds {
            small: 1_048_576,        // 1 MB
            medium: 104_857_600,     // 100 MB
            large: 1_073_741_824,    // 1 GB
        }
    }
}

/// How to handle a conflict (destination file already exists).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStrategy {
    /// Prompt the user for each conflict (GUI/interactive only).
    Ask,
    /// Skip the file; leave destination unchanged.
    Skip,
    /// Replace the existing destination file.
    Replace,
    /// Rename the incoming file to avoid collision (e.g. photo(1).jpg).
    Rename,
}

impl Default for ConflictStrategy {
    fn default() -> Self {
        ConflictStrategy::Ask
    }
}

/// The result of safety validation for a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum SafetyStatus {
    /// All safety checks passed.
    Safe,
    /// Checks passed with non-blocking warnings.
    Warnings { warnings: Vec<String> },
    /// Checks failed; plan cannot be executed.
    Blocked { errors: Vec<String> },
}

impl SafetyStatus {
    pub fn is_executable(&self) -> bool {
        !matches!(self, SafetyStatus::Blocked { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_thresholds_default() {
        let t = SizeThresholds::default();
        assert_eq!(t.small, 1_048_576);
        assert_eq!(t.medium, 104_857_600);
    }

    #[test]
    fn conflict_strategy_default_is_ask() {
        assert_eq!(ConflictStrategy::default(), ConflictStrategy::Ask);
    }

    #[test]
    fn safety_status_blocked_is_not_executable() {
        let s = SafetyStatus::Blocked {
            errors: vec!["bad".into()],
        };
        assert!(!s.is_executable());
    }

    #[test]
    fn safety_status_warnings_is_executable() {
        let s = SafetyStatus::Warnings {
            warnings: vec!["ok".into()],
        };
        assert!(s.is_executable());
    }

    #[test]
    fn organize_mode_serializes() {
        let m = OrganizeMode::ByDate {
            granularity: DateGranularity::Month,
        };
        let json = serde_json::to_string(&m).unwrap();
        assert!(json.contains("by_date"));
    }
}
