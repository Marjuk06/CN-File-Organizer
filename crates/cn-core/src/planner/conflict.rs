use crate::error::CnResult;
use crate::models::conflict::ConflictInfo;
use crate::models::operation::{FileOperation, OperationKind};
use std::collections::HashSet;
use std::path::PathBuf;

/// Detect conflicts in a list of operations.
///
/// A conflict occurs when:
/// 1. The destination file already exists on disk.
/// 2. Multiple operations in the same plan target the same destination path.
pub fn detect_conflicts(mut operations: Vec<FileOperation>) -> CnResult<Vec<FileOperation>> {
    let mut target_paths: HashSet<PathBuf> = HashSet::new();
    let mut in_flight_duplicates: HashSet<PathBuf> = HashSet::new();

    // Pass 1: Identify all destination paths within the plan
    // This helps us catch conflicts where two source files map to the same destination
    for op in &operations {
        if op.kind == OperationKind::Skip {
            continue;
        }

        if !target_paths.insert(op.destination.clone()) {
            in_flight_duplicates.insert(op.destination.clone());
        }
    }

    // Pass 2: Actually check against the disk and the in-flight duplicates
    for op in &mut operations {
        if op.kind == OperationKind::Skip {
            continue;
        }

        // Check 1: In-flight duplicate (multiple sources mapped to same destination)
        if in_flight_duplicates.contains(&op.destination) {
            // We treat this similar to an existing file conflict, using a dummy
            // "existing" file representing the collision in the plan itself.
            // A more sophisticated implementation might handle this separately.
            op.conflict = Some(ConflictInfo {
                existing_path: op.destination.clone(),
                existing_size: 0, // Unknown size for the pending other file
                existing_modified: None,
                incoming_path: op.source.clone(),
                incoming_size: op.size,
                incoming_modified: None,
                existing_hash: None,
                incoming_hash: None,
            });
            continue;
        }

        // Check 2: File already exists on disk
        if op.destination.exists() {
            let meta = match std::fs::metadata(&op.destination) {
                Ok(m) => m,
                Err(_) => continue, // If we can't read metadata, we might fail later, but skip conflict for now
            };

            let existing_size = meta.len();
            let existing_modified = meta.modified().ok().map(chrono::DateTime::from);

            let incoming_meta = std::fs::metadata(&op.source).ok();
            let incoming_modified = incoming_meta
                .and_then(|m| m.modified().ok())
                .map(chrono::DateTime::from);

            op.conflict = Some(ConflictInfo {
                existing_path: op.destination.clone(),
                existing_size,
                existing_modified,
                incoming_path: op.source.clone(),
                incoming_size: op.size,
                incoming_modified,
                existing_hash: None,
                incoming_hash: None,
            });
        }
    }

    Ok(operations)
}
