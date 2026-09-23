use cn_core::{
    models::{
        execution::FileOutcome,
        operation::{DateGranularity, OrganizeMode, SizeThresholds},
        scan_summary::ScanOptions,
    },
    planner::PlanOptions,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};

pub async fn run(
    path: PathBuf,
    mode_str: String,
    yes: bool,
    state_dir: &Path,
    is_json: bool,
) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let mode = match mode_str.to_lowercase().as_str() {
        "smart" => OrganizeMode::Smart,
        "extension" => OrganizeMode::ByExtension,
        "category" => OrganizeMode::ByCategory,
        "bydate" => OrganizeMode::ByDate {
            granularity: DateGranularity::Month,
        },
        "bysize" => OrganizeMode::BySize {
            thresholds: SizeThresholds::default(),
        },
        _ => anyhow::bail!(
            "Invalid organize mode. Use 'smart', 'extension', 'category', 'bydate', 'bysize'."
        ),
    };

    if !is_json {
        println!("Scanning directory: {}...", path.display());
    }

    let options = ScanOptions::default();
    // In executor we don't care about recursive for now or just set default
    let summary = cn_core::scanner::scan_directory(&path, options, None).await?;

    let dest = path.clone();
    let plan_options = PlanOptions {
        destination: Some(dest),
        ..Default::default()
    };
    let plan = cn_core::planner::build_plan(&summary, mode, plan_options)?;

    if plan.operations.is_empty() {
        if is_json {
            println!(r#"{{"status": "No actions needed"}}"#);
        } else {
            println!("No files to move. Directory is already organized.");
        }
        return Ok(());
    }

    if !is_json {
        println!(
            "\nPlan ready: {} files ({} bytes) will be moved.",
            plan.total_files(),
            plan.estimated_bytes
        );
    }

    if !yes && !is_json {
        println!("Execute this plan? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let pb = if !is_json {
        let pb = ProgressBar::new(plan.estimated_bytes);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };

    let pb_clone = pb.clone();

    // Create channel for progress updates
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    // Spawn task to update progress bar
    tokio::spawn(async move {
        let mut current_bytes = 0;
        while let Some(progress) = rx.recv().await {
            if let Some(pb) = &pb_clone {
                if let cn_core::executor::ExecuteProgressEvent::Finished {
                    bytes_processed, ..
                } = progress
                {
                    current_bytes += bytes_processed;
                    pb.set_position(current_bytes);
                }
            }
        }
    });

    let cancel_token = cn_core::executor::cancel::CancellationToken::new();
    let result = cn_core::executor::execute_plan(plan, state_dir, cancel_token, Some(tx)).await?;

    // Save the execution result so `undo` can use it later
    let result_path = state_dir.join(format!("result_{}.json", result.operation_id));
    if let Ok(json) = serde_json::to_string_pretty(&result) {
        let _ = std::fs::write(&result_path, json);
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Done");
    }

    if is_json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("\nExecution Complete!");
        println!("Files Processed: {}", result.file_results.len());
        println!("Succeeded:       {}", result.succeeded);
        println!("Skipped:         {}", result.skipped);
        println!("Failed:          {}", result.failed);

        let errors: Vec<_> = result
            .file_results
            .iter()
            .filter(|r| r.outcome == FileOutcome::Failed)
            .collect();
        if !errors.is_empty() {
            println!("Errors encountered: {}", errors.len());
            for err in errors.iter().take(5) {
                if let Some(err_msg) = &err.error {
                    println!("  - {}: {}", err.source.display(), err_msg);
                }
            }
            if errors.len() > 5 {
                println!("  ... and {} more", errors.len() - 5);
            }
        }
    }

    Ok(())
}
