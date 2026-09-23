use cn_core::{
    planner::PlanOptions,
    models::{
        scan_summary::ScanOptions,
        operation::OrganizeMode,
    },
};
use std::path::PathBuf;

pub async fn run(
    path: PathBuf,
    mode_str: String,
    destination: Option<PathBuf>,
    is_json: bool,
) -> anyhow::Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let mode = match mode_str.to_lowercase().as_str() {
        "smart" => OrganizeMode::Smart,
        "extension" => OrganizeMode::ByExtension,
        "category" => OrganizeMode::ByCategory,
        _ => anyhow::bail!("Invalid organize mode. Use 'smart', 'extension', or 'category'."),
    };

    let mut options = ScanOptions::default();
    let summary = cn_core::scanner::scan_directory(&path, options, None).await?;

    if !is_json {
        println!("Generating organization plan (Mode: {:?})...", mode);
    }

    let dest = destination.unwrap_or_else(|| path.clone());
    let mut plan_options = PlanOptions::default();
    plan_options.destination = Some(dest);
    let plan = cn_core::planner::build_plan(&summary, mode, plan_options)?;

    if is_json {
        println!("{}", serde_json::to_string_pretty(&plan)?);
    } else {
        println!("\nPlan Generated Successfully!");
        println!("--------------------------------------------------");
        println!("Source:       {}", plan.source_dir.display());
        println!("Destination:  {}", plan.destination_dir.display());
        println!("Total Files:  {}", plan.total_files());
        println!("Total Size:   {} bytes", plan.estimated_bytes);
        
        let conflicts = plan.operations.iter().filter(|op| op.conflict.is_some()).count();
        println!("Conflicts:    {}", conflicts);
        
        if conflicts == 0 {
            println!("\nThis plan is safe to execute.");
        } else {
            println!("\nWARNING: There are {} conflicting files that will be skipped or renamed.", conflicts);
        }
        
        println!("\nRun `organize execute {}` to apply these changes.", path.display());
    }

    Ok(())
}
