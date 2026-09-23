use crate::models::operation::{FileOperation, OperationKind};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Builds a text-based tree preview of the intended operations.
/// Helpful for CLI interfaces to show users what will happen.
pub fn build_tree_preview(
    _source_dir: &Path,
    _destination_dir: &Path,
    operations: &[FileOperation],
) -> String {
    let mut tree: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for op in operations {
        if op.kind == OperationKind::Skip {
            continue;
        }

        let parent = op
            .destination
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .to_string();
            
        let file_name = op
            .destination
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        tree.entry(parent).or_default().push(file_name);
    }

    let mut output = String::new();
    for (folder, files) in tree {
        output.push_str(&format!("📁 {}\n", folder));
        for file in files {
            output.push_str(&format!("  ├── 📄 {}\n", file));
        }
    }
    
    if output.is_empty() {
        output.push_str("No files to organize.\n");
    }

    output
}
