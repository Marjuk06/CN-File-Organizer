pub mod smart;
pub mod modes;
pub mod conflict;
pub mod preview;
pub mod validation;

use crate::error::CnResult;
use crate::models::operation::{ConflictStrategy, OperationPlan, OrganizeMode};
use crate::models::scan_summary::ScanSummary;
use std::path::PathBuf;

/// Options for building an operation plan.
#[derive(Debug, Clone)]
pub struct PlanOptions {
    /// Where to put organized files. None = in-place (subfolders inside source).
    pub destination: Option<PathBuf>,
    pub conflict_strategy: ConflictStrategy,
}

impl Default for PlanOptions {
    fn default() -> Self {
        PlanOptions {
            destination: None,
            conflict_strategy: ConflictStrategy::Ask,
        }
    }
}

/// Build an OperationPlan from a ScanSummary.
pub fn build_plan(
    summary: &ScanSummary,
    mode: OrganizeMode,
    opts: PlanOptions,
) -> CnResult<OperationPlan> {
    let destination_dir = opts
        .destination
        .clone()
        .unwrap_or_else(|| summary.source_dir.clone());

    // Detect cross-filesystem
    let cross_filesystem = crate::safety::boundaries::same_filesystem(
        &summary.source_dir,
        &destination_dir,
    ) == false;

    // Build file operations based on mode
    let operations = match &mode {
        OrganizeMode::Smart => smart::build_operations(summary, &destination_dir)?,
        OrganizeMode::ByCategory => modes::by_category(summary, &destination_dir)?,
        OrganizeMode::ByExtension => modes::by_extension(summary, &destination_dir)?,
        OrganizeMode::ByDate { granularity } => {
            modes::by_date(summary, &destination_dir, *granularity)?
        }
        OrganizeMode::BySize { thresholds } => {
            modes::by_size(summary, &destination_dir, thresholds)?
        }
        OrganizeMode::CustomRules => {
            // Rules mode: use loaded rules from config
            // For now return empty — rule engine wires this in
            vec![]
        }
    };

    // Detect conflicts
    let operations = conflict::detect_conflicts(operations)?;

    // Estimate total bytes
    let estimated_bytes: u64 = operations.iter().map(|op| op.size).sum();

    // Run safety validation
    let safety_status = validation::validate(
        &summary.source_dir,
        &destination_dir,
        &operations,
        estimated_bytes,
    );

    Ok(OperationPlan {
        id: uuid::Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        source_dir: summary.source_dir.clone(),
        destination_dir,
        mode,
        operations,
        conflict_strategy: opts.conflict_strategy,
        estimated_bytes,
        cross_filesystem,
        safety_status,
    })
}
