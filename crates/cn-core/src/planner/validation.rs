use crate::models::operation::{FileOperation, OperationKind, SafetyStatus};
use std::path::Path;

/// Validates the operations and returns a SafetyStatus.
pub fn validate(
    _source_dir: &Path,
    destination_dir: &Path,
    operations: &[FileOperation],
    estimated_bytes: u64,
) -> SafetyStatus {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Canonicalize destination_dir if possible, otherwise use absolute
    let dest_canon = destination_dir
        .canonicalize()
        .unwrap_or_else(|_| destination_dir.to_path_buf());

    // 1. Check disk space
    if let Some(available) = crate::safety::permissions::available_bytes(destination_dir) {
        if estimated_bytes > available {
            errors.push(format!(
                "Insufficient disk space. Needed: {}, Available: {}",
                estimated_bytes, available
            ));
        } else if available - estimated_bytes < 100 * 1024 * 1024 {
            // Less than 100MB remaining
            warnings.push("Available disk space will be critically low after operation.".into());
        }
    }

    // 2. Check for protected paths in operations
    for op in operations {
        if op.kind == OperationKind::Skip {
            continue;
        }

        if crate::safety::protected::is_protected(&op.destination) {
            errors.push(format!(
                "Destination path is protected: {}",
                op.destination.display()
            ));
        }

        // Path Traversal Security Check: Ensure destination is inside destination_dir
        // We do this by checking if the destination starts with the canonical destination_dir.
        // For paths that don't exist yet, we check the parent.
        let mut check_path = op.destination.clone();
        while !check_path.exists() && check_path.parent().is_some() {
            check_path = check_path.parent().unwrap().to_path_buf();
        }
        let op_canon = check_path
            .canonicalize()
            .unwrap_or_else(|_| check_path.to_path_buf());
        if !op_canon.starts_with(&dest_canon) {
            errors.push(format!(
                "Security Exception: Path traversal detected. Destination escapes root: {}",
                op.destination.display()
            ));
        }
    }

    // 3. No operations check
    let active_ops = operations
        .iter()
        .filter(|op| op.kind != OperationKind::Skip)
        .count();
    if active_ops == 0 {
        warnings.push("This plan will result in no files being moved or copied.".into());
    }

    if !errors.is_empty() {
        SafetyStatus::Blocked { errors }
    } else if !warnings.is_empty() {
        SafetyStatus::Warnings { warnings }
    } else {
        SafetyStatus::Safe
    }
}
