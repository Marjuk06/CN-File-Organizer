use cn_core::models::operation::{OperationPlan, OrganizeMode};
use cn_core::models::scan_summary::ScanSummary;
use cn_core::planner::{build_plan, PlanOptions};
use std::path::PathBuf;

#[tauri::command(async)]
pub async fn create_plan(
    summary: ScanSummary,
    mode: OrganizeMode,
    destination: Option<String>,
) -> Result<OperationPlan, String> {
    let mut opts = PlanOptions::default();
    if let Some(dest) = destination {
        opts.destination = Some(PathBuf::from(dest));
    }

    tokio::task::spawn_blocking(move || build_plan(&summary, mode, opts).map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}
