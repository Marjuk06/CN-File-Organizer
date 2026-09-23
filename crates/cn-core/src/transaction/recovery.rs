use crate::error::CnResult;
use crate::transaction::journal::{JournalEntry, TransactionJournal};
use std::collections::HashSet;

/// Evaluates the journal to determine if recovery is needed and performs it.
pub struct RecoveryEngine;

impl RecoveryEngine {
    /// Check the journal and recover any interrupted operations.
    /// Returns the number of files rolled back.
    pub fn recover(journal: &TransactionJournal) -> CnResult<u64> {
        let entries = journal.read_all()?;
        if entries.is_empty() {
            return Ok(0);
        }

        // We only care about operations that started but didn't finish.
        let mut started = HashSet::new();
        let mut finished = HashSet::new();

        // Track the last known state of each file move
        // Key: (operation_id, source, destination)
        let mut moving = HashSet::new();
        let mut moved = HashSet::new();

        for entry in entries {
            match entry {
                JournalEntry::Start { operation_id, .. } => {
                    started.insert(operation_id);
                }
                JournalEntry::Success { operation_id }
                | JournalEntry::Failed { operation_id, .. } => {
                    finished.insert(operation_id);
                }
                JournalEntry::FileMoving {
                    operation_id,
                    source,
                    destination,
                } => {
                    moving.insert((operation_id, source, destination));
                }
                JournalEntry::FileMoved {
                    operation_id,
                    source,
                    destination,
                } => {
                    moved.insert((operation_id, source, destination));
                }
            }
        }

        let mut recovered_count = 0;

        // Find operations that were interrupted (started but not finished)
        let interrupted: Vec<_> = started.difference(&finished).collect();
        for &op_id in interrupted {
            // Find files that started moving but didn't complete
            for (id, src, dst) in &moving {
                if id == &op_id && !moved.contains(&(*id, src.clone(), dst.clone())) {
                    // This file was interrupted mid-move.
                    // If it was a cross-filesystem copy, the destination might be partially written.
                    // The safest recovery is to delete the partially written destination if the source still exists.

                    if src.exists() && dst.exists() {
                        // The file didn't finish moving, so the source is our source of truth.
                        // We delete the partially copied destination.
                        let _ = std::fs::remove_file(dst);
                        recovered_count += 1;
                    }
                }
            }
        }

        Ok(recovered_count)
    }
}
