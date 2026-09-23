use crate::error::CnResult;
use crate::models::file_info::FileKind;
use crate::models::operation::{DateGranularity, FileOperation, OperationKind, SizeThresholds};
use crate::models::scan_summary::ScanSummary;
use std::path::Path;
use uuid::Uuid;

/// Group files by their detected category.
pub fn by_category(summary: &ScanSummary, destination_dir: &Path) -> CnResult<Vec<FileOperation>> {
    // Same as smart but without automatic multi-category optimization
    crate::planner::smart::build_operations(summary, destination_dir)
}

/// Group files by lowercase file extension.
pub fn by_extension(summary: &ScanSummary, destination_dir: &Path) -> CnResult<Vec<FileOperation>> {
    let mut ops = Vec::new();

    for file in &summary.files {
        if file.kind != FileKind::Regular {
            continue;
        }

        let subfolder = file
            .extension
            .as_deref()
            .map(|e| e.to_uppercase())
            .unwrap_or_else(|| "No Extension".to_string());

        let destination = destination_dir
            .join(&subfolder)
            .join(file.path.file_name().unwrap_or_default());

        ops.push(FileOperation {
            id: Uuid::new_v4(),
            source: file.path.clone(),
            destination,
            kind: OperationKind::Move,
            category: file.category,
            size: file.size,
            conflict: None,
        });
    }

    Ok(ops)
}

/// Group files by modification date.
pub fn by_date(
    summary: &ScanSummary,
    destination_dir: &Path,
    granularity: DateGranularity,
) -> CnResult<Vec<FileOperation>> {
    let mut ops = Vec::new();

    for file in &summary.files {
        if file.kind != FileKind::Regular {
            continue;
        }

        let folder_name = if let Some(modified) = file.modified_at {
            match granularity {
                DateGranularity::Year => modified.format("%Y").to_string(),
                DateGranularity::Month => modified.format("%Y-%m").to_string(),
                DateGranularity::Day => modified.format("%Y-%m-%d").to_string(),
            }
        } else {
            "Unknown Date".to_string()
        };

        let destination = destination_dir
            .join(&folder_name)
            .join(file.path.file_name().unwrap_or_default());

        ops.push(FileOperation {
            id: Uuid::new_v4(),
            source: file.path.clone(),
            destination,
            kind: OperationKind::Move,
            category: file.category,
            size: file.size,
            conflict: None,
        });
    }

    Ok(ops)
}

/// Group files by size range (Small / Medium / Large / Huge).
pub fn by_size(
    summary: &ScanSummary,
    destination_dir: &Path,
    thresholds: &SizeThresholds,
) -> CnResult<Vec<FileOperation>> {
    let mut ops = Vec::new();

    for file in &summary.files {
        if file.kind != FileKind::Regular {
            continue;
        }

        let label = if file.size < thresholds.small {
            "Small"
        } else if file.size < thresholds.medium {
            "Medium"
        } else if file.size < thresholds.large {
            "Large"
        } else {
            "Huge"
        };

        let destination = destination_dir
            .join(label)
            .join(file.path.file_name().unwrap_or_default());

        ops.push(FileOperation {
            id: Uuid::new_v4(),
            source: file.path.clone(),
            destination,
            kind: OperationKind::Move,
            category: file.category,
            size: file.size,
            conflict: None,
        });
    }

    Ok(ops)
}
