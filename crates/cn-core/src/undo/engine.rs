use crate::error::{CnError, CnResult};
use crate::models::execution::{ExecutionResult, FileOutcome};
use std::fs;
use std::path::Path;

/// Undo a completed execution.
///
/// This goes through the `ExecutionResult` and reverts every file move.
/// Returns the number of files successfully restored.
pub fn undo_execution(result: &ExecutionResult) -> CnResult<u64> {
    if !result.is_success() {
        return Err(CnError::NotUndoable("Only successful or partially successful operations can be undone".into()));
    }

    let mut restored = 0;

    for file_res in &result.file_results {
        // We only need to undo files that were actually moved or renamed
        if file_res.outcome == FileOutcome::Moved || file_res.outcome == FileOutcome::Renamed {
            
            // Safety check: Has the file been modified since we moved it?
            if let Ok(meta) = fs::metadata(&file_res.destination) {
                let current_modified: Option<chrono::DateTime<chrono::Utc>> = meta.modified().ok().map(chrono::DateTime::from);
                
                // If it was modified after we put it there, don't undo to prevent data loss
                // We use a small epsilon for filesystem timestamp inaccuracies
                if let (Some(current), Some(original)) = (current_modified, file_res.destination_modified_at) {
                    if current.timestamp_millis() - original.timestamp_millis() > 2000 {
                        // File was modified, skip undo for this file
                        continue;
                    }
                }
            } else {
                // File disappeared
                continue;
            }
            
            // Ensure original parent directory exists
            if let Some(parent) = file_res.source.parent() {
                let _ = fs::create_dir_all(parent);
            }
            
            // Move it back
            // Note: cross-filesystem undo is just a copy+delete since it's the reverse of cross-fs move
            // We use standard fs::rename here; if cross-fs, standard fs::rename might fail,
            // so we should fallback to copy+delete
            if fs::rename(&file_res.destination, &file_res.source).is_err() {
                if fs::copy(&file_res.destination, &file_res.source).is_ok() {
                    let _ = fs::remove_file(&file_res.destination);
                    restored += 1;
                }
            } else {
                restored += 1;
            }
        }
    }

    // Optional: Clean up empty directories left behind by the undo
    cleanup_empty_dirs(result);

    Ok(restored)
}

fn cleanup_empty_dirs(result: &ExecutionResult) {
    // Collect unique directories we moved files *to*
    let mut dirs_to_check = std::collections::HashSet::new();
    for file_res in &result.file_results {
        if let Some(parent) = file_res.destination.parent() {
            dirs_to_check.insert(parent.to_path_buf());
        }
    }

    for dir in dirs_to_check {
        // fs::remove_dir only succeeds if the directory is empty
        let _ = fs::remove_dir(&dir);
        // Also try removing its parent if it's empty (e.g. "Images/2026")
        if let Some(parent) = dir.parent() {
            let _ = fs::remove_dir(parent);
        }
    }
}
