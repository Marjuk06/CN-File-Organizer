use crate::error::CnResult;
use crate::models::file_info::FileKind;
use crate::models::operation::{FileOperation, OperationKind};
use crate::models::scan_summary::ScanSummary;
use std::path::Path;
use uuid::Uuid;

/// Smart organize: group files by their detected category.
/// This is the most user-friendly default mode.
pub fn build_operations(
    summary: &ScanSummary,
    destination_dir: &Path,
) -> CnResult<Vec<FileOperation>> {
    let mut ops = Vec::new();

    for file in &summary.files {
        if file.kind != FileKind::Regular {
            continue;
        }

        let category = match &file.category {
            Some(c) => c,
            None => continue,
        };

        let subfolder = category.folder_name();
        let destination = destination_dir
            .join(subfolder)
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
