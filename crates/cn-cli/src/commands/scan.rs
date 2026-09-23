use cn_core::models::scan_summary::ScanOptions;
use std::path::PathBuf;

pub async fn run(path: PathBuf, max_depth: Option<usize>, include_hidden: bool, is_json: bool) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let mut options = ScanOptions::default();
    options.include_hidden = include_hidden;
    options.max_depth = max_depth;

    if !is_json {
        println!("Scanning directory: {}...", path.display());
    }

    let summary = cn_core::scanner::scan_directory(&path, options, None).await?;

    if is_json {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        println!("\nScan Summary for '{}':", summary.source_dir.display());
        println!("--------------------------------------------------");
        println!("Total Files: {} ({} skipped)", summary.total_files, summary.hidden_skipped);
        println!("Total Size:  {} bytes", summary.total_bytes);
        println!("\nCategories:");
        for (category, stats) in &summary.by_category {
            println!("  {:<12} {} ({} bytes)", category, stats.count, stats.total_bytes);
        }
    }

    Ok(())
}
