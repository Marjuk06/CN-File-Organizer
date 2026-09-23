pub mod walk;
pub mod metadata;
pub mod symlink;

use crate::error::CnResult;
use crate::models::scan_summary::{ScanOptions, ScanSummary};
use std::path::Path;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Progress events emitted during a directory scan.
#[derive(Debug, Clone)]
pub enum ScanProgressEvent {
    /// A new file was discovered.
    FileFound { count: u64, path: String },
    /// Scan is complete.
    Complete,
}

pub type ProgressSender = mpsc::Sender<ScanProgressEvent>;

/// Scan a directory and return a ScanSummary.
///
/// Progress events are sent via `tx`. Pass a channel with sufficient buffer.
/// This function runs synchronously — wrap in `tokio::task::spawn_blocking` for async contexts.
pub async fn scan_directory(
    path: &Path,
    opts: ScanOptions,
    tx: Option<ProgressSender>,
) -> CnResult<ScanSummary> {
    walk::scan(path, opts, tx).await
}
