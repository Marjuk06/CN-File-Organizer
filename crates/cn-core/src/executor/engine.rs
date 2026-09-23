use crate::error::CnResult;
use crate::executor::cancel::CancellationToken;
use crate::executor::mover::execute_operation;
use crate::executor::progress::{ExecuteProgressEvent, ProgressSender};
use crate::models::execution::{ExecutionResult, ExecutionStatus, FileOutcome};
use crate::models::operation::OperationPlan;
use crate::transaction::journal::{JournalEntry, TransactionJournal};
use std::path::Path;
use tokio::time::Instant;

/// Orchestrates the execution of a complete operation plan.
pub async fn execute_plan(
    plan: OperationPlan,
    state_dir: &Path,
    cancel_token: CancellationToken,
    progress_tx: Option<ProgressSender>,
) -> CnResult<ExecutionResult> {
    let start_time = Instant::now();
    let mut journal = TransactionJournal::new(state_dir);
    journal.open()?;

    // Log start of operation
    journal.append(&JournalEntry::Start {
        operation_id: plan.id,
        plan_path: state_dir.join(format!("plan_{}.json", plan.id)),
    })?;

    let mut succeeded = 0;
    let mut skipped = 0;
    let mut failed: u64 = 0;
    let mut file_results = Vec::new();
    let mut execution_status = ExecutionStatus::Completed;

    for op in plan.operations {
        if cancel_token.is_cancelled() {
            execution_status = ExecutionStatus::Cancelled;
            break;
        }

        // Notify progress (start)
        if let Some(tx) = &progress_tx {
            let _ = tx.try_send(ExecuteProgressEvent::Started {
                operation_id: op.id,
                source: op.source.to_string_lossy().to_string(),
                destination: op.destination.to_string_lossy().to_string(),
            });
        }

        // Log to journal (moving)
        journal.append(&JournalEntry::FileMoving {
            operation_id: op.id,
            source: op.source.clone(),
            destination: op.destination.clone(),
        })?;

        // Perform the move
        // Note: this block is synchronous blocking IO, wrap execute_plan in spawn_blocking
        let result = execute_operation(&op, &plan.conflict_strategy, plan.cross_filesystem);

        match result.outcome {
            FileOutcome::Moved | FileOutcome::Renamed => {
                succeeded += 1;
                journal.append(&JournalEntry::FileMoved {
                    operation_id: op.id,
                    source: op.source.clone(),
                    destination: result.destination.clone(),
                })?;
            }
            FileOutcome::Skipped => {
                skipped += 1;
            }
            FileOutcome::Failed => {
                failed += 1;
                // If any file fails, the whole operation is considered PartialSuccess at best
                if execution_status == ExecutionStatus::Completed {
                    execution_status = ExecutionStatus::PartialSuccess;
                }
            }
        }

        // Notify progress (finished)
        if let Some(tx) = &progress_tx {
            let _ = tx.try_send(ExecuteProgressEvent::Finished {
                operation_id: op.id,
                outcome: result.outcome.clone(),
                bytes_processed: op.size,
            });
        }

        file_results.push(result);
    }

    if failed as usize == file_results.len() && !file_results.is_empty() && execution_status != ExecutionStatus::Cancelled {
        execution_status = ExecutionStatus::Failed;
    }

    // Log completion
    if execution_status == ExecutionStatus::Completed || execution_status == ExecutionStatus::PartialSuccess {
        journal.append(&JournalEntry::Success { operation_id: plan.id })?;
        journal.clear()?; // Success means we don't need recovery data
    } else {
        journal.append(&JournalEntry::Failed { 
            operation_id: plan.id, 
            reason: format!("{:?}", execution_status) 
        })?;
    }

    if let Some(tx) = &progress_tx {
        let _ = tx.try_send(ExecuteProgressEvent::Complete);
    }

    Ok(ExecutionResult {
        operation_id: plan.id,
        succeeded,
        skipped,
        failed,
        file_results,
        status: execution_status,
        duration_ms: start_time.elapsed().as_millis() as u64,
    })
}
