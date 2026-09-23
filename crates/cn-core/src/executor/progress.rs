use crate::models::execution::FileOutcome;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Progress events emitted during plan execution.
#[derive(Debug, Clone)]
pub enum ExecuteProgressEvent {
    /// A single file started processing.
    Started {
        operation_id: Uuid,
        source: String,
        destination: String,
    },
    /// A single file finished processing.
    Finished {
        operation_id: Uuid,
        outcome: FileOutcome,
        bytes_processed: u64,
    },
    /// The entire execution is complete.
    Complete,
    /// An error occurred that aborted the execution.
    Error(String),
}

pub type ProgressSender = mpsc::Sender<ExecuteProgressEvent>;
