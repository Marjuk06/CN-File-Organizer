use cn_core::models::scan_summary::{ScanOptions, ScanSummary};
use std::path::PathBuf;

#[tauri::command(async)]
pub async fn scan_directory(
    path: String,
    options: Option<ScanOptions>,
) -> Result<ScanSummary, String> {
    let path_buf = PathBuf::from(&path);
    let opts = options.unwrap_or_default();

    cn_core::scanner::walk::scan(&path_buf, opts, None)
        .await
        .map_err(|e| e.to_string())
}
