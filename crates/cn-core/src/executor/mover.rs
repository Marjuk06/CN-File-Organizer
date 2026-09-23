use crate::error::{CnError, CnResult};
use crate::models::conflict::ConflictInfo;
use crate::models::execution::{FileOutcome, FileResult};
use crate::models::operation::{ConflictStrategy, FileOperation, OperationKind};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Execute a single file operation.
pub fn execute_operation(
    op: &FileOperation,
    strategy: &ConflictStrategy,
    cross_fs: bool,
) -> FileResult {
    let mut outcome = FileOutcome::Failed;
    let mut error = None;
    let mut verified_hash = None;
    let mut destination = op.destination.clone();

    // 1. Handle Skip intent
    if op.kind == OperationKind::Skip {
        return FileResult {
            operation_id: op.id,
            source: op.source.clone(),
            destination,
            outcome: FileOutcome::Skipped,
            error: None,
            verified_hash: None,
            destination_modified_at: None,
        };
    }

    // 2. Ensure parent directory exists
    if let Some(parent) = destination.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return FileResult {
                operation_id: op.id,
                source: op.source.clone(),
                destination,
                outcome: FileOutcome::Failed,
                error: Some(format!("Failed to create destination directory: {}", e)),
                verified_hash: None,
                destination_modified_at: None,
            };
        }
    }

    // 3. Handle Conflicts (already present on disk)
    if destination.exists() {
        match strategy {
            ConflictStrategy::Ask => {
                // Should have been resolved by UI before this point. If not, fail safe.
                error = Some("Unresolved conflict".into());
                outcome = FileOutcome::Failed;
            }
            ConflictStrategy::Skip => {
                outcome = FileOutcome::Skipped;
            }
            ConflictStrategy::Replace => {
                // Delete existing first
                if let Err(e) = fs::remove_file(&destination) {
                    error = Some(format!("Failed to remove existing file: {}", e));
                    outcome = FileOutcome::Failed;
                }
            }
            ConflictStrategy::Rename => {
                destination = ConflictInfo::generate_rename(&destination);
                outcome = FileOutcome::Renamed; // Will upgrade to Moved on success
            }
        }
    }

    if outcome == FileOutcome::Failed || outcome == FileOutcome::Skipped {
        return FileResult {
            operation_id: op.id,
            source: op.source.clone(),
            destination,
            outcome,
            error,
            verified_hash: None,
            destination_modified_at: None,
        };
    }

    // 4. Perform the actual move/copy
    let res = if op.kind == OperationKind::Copy {
        copy_file(&op.source, &destination)
    } else if cross_fs {
        move_cross_fs(&op.source, &destination)
    } else {
        move_same_fs(&op.source, &destination)
    };

    match res {
        Ok(hash) => {
            // Outcome is either Moved or Renamed
            if outcome != FileOutcome::Renamed {
                outcome = FileOutcome::Moved;
            }
            verified_hash = hash;
        }
        Err(e) => {
            outcome = FileOutcome::Failed;
            error = Some(e.to_string());
        }
    }

    let meta = fs::metadata(&destination).ok();
    let destination_modified_at = meta
        .and_then(|m| m.modified().ok())
        .map(chrono::DateTime::from);

    FileResult {
        operation_id: op.id,
        source: op.source.clone(),
        destination,
        outcome,
        error,
        verified_hash,
        destination_modified_at,
    }
}

/// Simple standard copy.
fn copy_file(source: &Path, destination: &Path) -> CnResult<Option<String>> {
    fs::copy(source, destination)?;
    Ok(None)
}

/// Simple rename (atomic on same filesystem).
fn move_same_fs(source: &Path, destination: &Path) -> CnResult<Option<String>> {
    fs::rename(source, destination)?;
    Ok(None)
}

/// Cross-filesystem move: Copy -> Verify SHA-256 -> Delete original.
fn move_cross_fs(source: &Path, destination: &Path) -> CnResult<Option<String>> {
    // 1. Hash source
    let source_hash = hash_file(source)?;

    // 2. Copy
    fs::copy(source, destination)?;

    // 3. Hash destination
    let dest_hash = match hash_file(destination) {
        Ok(h) => h,
        Err(e) => {
            // Clean up destination if verification fails to even run
            let _ = fs::remove_file(destination);
            return Err(e);
        }
    };

    // 4. Verify
    if source_hash != dest_hash {
        let _ = fs::remove_file(destination);
        return Err(CnError::VerificationFailed(source.to_string_lossy().to_string()));
    }

    // 5. Delete source
    fs::remove_file(source)?;

    Ok(Some(dest_hash))
}

fn hash_file(path: &Path) -> CnResult<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 65536];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
