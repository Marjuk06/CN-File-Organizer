pub mod protected;
pub mod boundaries;
pub mod permissions;

use crate::error::{CnError, CnResult};
use crate::models::operation::OperationPlan;
use std::path::Path;

/// Run all pre-flight safety checks against a plan.
/// Returns Ok(()) if the plan is safe to execute.
/// Returns an error if any blocking check fails.
pub fn validate_plan_safety(plan: &OperationPlan) -> CnResult<Vec<String>> {
    let mut warnings: Vec<String> = Vec::new();

    // 1. Check source directory exists
    if !plan.source_dir.exists() {
        return Err(CnError::SourceNotFound(
            plan.source_dir.to_string_lossy().to_string(),
        ));
    }

    // 2. Check destination is writable
    permissions::check_writable(&plan.destination_dir)?;

    // 3. Check source/destination overlap
    check_no_overlap(&plan.source_dir, &plan.destination_dir)?;

    // 4. Check all operation sources
    for op in &plan.operations {
        // Protected path check on destinations
        if protected::is_protected(&op.destination) {
            return Err(CnError::ProtectedPath(
                op.destination.to_string_lossy().to_string(),
            ));
        }
    }

    // 5. Cross-filesystem warning
    if plan.cross_filesystem {
        warnings.push(
            "Source and destination are on different filesystems. \
             Files will be copied then verified before deletion."
                .to_string(),
        );
    }

    Ok(warnings)
}

/// Check that source and destination do not overlap.
fn check_no_overlap(source: &Path, destination: &Path) -> CnResult<()> {
    // Destination cannot be the same as source (in-place handled correctly — subfolders are fine)
    // Destination cannot be an ancestor of source
    // Source cannot be an ancestor of destination when they are different directories

    let src = source
        .canonicalize()
        .unwrap_or_else(|_| source.to_path_buf());
    let dst = destination
        .canonicalize()
        .unwrap_or_else(|_| destination.to_path_buf());

    // destination is inside source: this is fine (in-place mode creates subfolders)
    // source is inside destination: this is a problem
    if src.starts_with(&dst) && src != dst {
        return Err(CnError::SelfContained(format!(
            "Source '{}' is inside destination '{}'",
            src.display(),
            dst.display()
        )));
    }

    Ok(())
}
