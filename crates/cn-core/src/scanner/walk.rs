use crate::error::CnResult;

use crate::models::file_info::{FileInfo, FileKind};
use crate::models::scan_summary::{CategoryStats, ScanOptions, ScanSummary};
use crate::scanner::ScanProgressEvent;
use crate::scanner::ProgressSender;
use crate::classifier;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;
use walkdir::WalkDir;

/// Core directory walk + classification.
/// Returns a complete ScanSummary.
pub async fn scan(
    path: &Path,
    opts: ScanOptions,
    tx: Option<ProgressSender>,
) -> CnResult<ScanSummary> {
    let source_dir = path.to_path_buf();
    let mut files: Vec<FileInfo> = Vec::new();
    let mut total_bytes: u64 = 0;
    let mut symlink_count: u64 = 0;
    let mut broken_symlink_count: u64 = 0;
    let mut special_file_count: u64 = 0;
    let mut hidden_skipped: u64 = 0;

    let walker = WalkDir::new(path)
        .follow_links(opts.follow_symlinks)
        .min_depth(1)
        .max_depth(opts.max_depth.unwrap_or(usize::MAX));

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue, // permission denied or other IO error — skip, don't fail
        };

        let file_name = entry.file_name().to_string_lossy();

        // Skip hidden files unless opted in
        if !opts.include_hidden && file_name.starts_with('.') && entry.depth() > 0 {
            if entry.file_type().is_file() {
                hidden_skipped += 1;
            }
            continue;
        }

        // Only process files (not directories)
        if entry.file_type().is_dir() {
            continue;
        }

        let path_buf = entry.path().to_path_buf();

        // Check for symlinks
        if entry.path_is_symlink() {
            // Determine if symlink is broken
            let is_broken = std::fs::metadata(entry.path()).is_err();
            if is_broken {
                broken_symlink_count += 1;
                files.push(FileInfo {
                    path: path_buf,
                    size: 0,
                    kind: FileKind::BrokenSymlink,
                    category: None,
                    mime_type: None,
                    extension: None,
                    modified_at: None,
                    created_at: None,
                    is_hidden: file_name.starts_with('.'),
                });
            } else {
                symlink_count += 1;
                files.push(FileInfo {
                    path: path_buf,
                    size: 0,
                    kind: FileKind::Symlink,
                    category: None,
                    mime_type: None,
                    extension: None,
                    modified_at: None,
                    created_at: None,
                    is_hidden: file_name.starts_with('.'),
                });
            }
            continue;
        }

        // Get metadata
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        // Check for special files (device, FIFO, socket)
        if !meta.is_file() {
            special_file_count += 1;
            continue;
        }

        let size = meta.len();
        let modified_at = meta
            .modified()
            .ok()
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t));
        let created_at = meta
            .created()
            .ok()
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t));

        let extension = path_buf
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        // Classify the file
        let (category, mime_type) = classifier::classify(&path_buf);

        total_bytes += size;

        let file_info = FileInfo {
            path: path_buf.clone(),
            size,
            kind: FileKind::Regular,
            category: Some(category),
            mime_type,
            extension,
            modified_at,
            created_at,
            is_hidden: file_name.starts_with('.'),
        };

        files.push(file_info);

        // Send progress event
        let count = files.len() as u64;
        if let Some(ref sender) = tx {
            let _ = sender
                .try_send(ScanProgressEvent::FileFound {
                    count,
                    path: path_buf.to_string_lossy().to_string(),
                });
        }
    }

    // Build category stats
    let mut by_category: HashMap<String, CategoryStats> = HashMap::new();
    for f in &files {
        if let Some(cat) = &f.category {
            let stats = by_category
                .entry(cat.folder_name().to_string())
                .or_default();
            stats.count += 1;
            stats.total_bytes += f.size;
        }
    }

    if let Some(sender) = tx {
        let _ = sender.try_send(ScanProgressEvent::Complete);
    }

    let total_files = files.iter().filter(|f| f.kind == FileKind::Regular).count() as u64;

    Ok(ScanSummary {
        id: Uuid::new_v4(),
        source_dir,
        files,
        total_files,
        total_bytes,
        by_category,
        symlink_count,
        broken_symlink_count,
        special_file_count,
        hidden_skipped,
        options: opts,
    })
}
