use cn_core::{
    history::HistoryStore,
    models::{execution::ExecutionResult, history::OperationStatus},
    undo::undo_execution,
};
use std::path::Path;
use uuid::Uuid;

pub async fn run(
    operation_id: Option<String>,
    history: &HistoryStore,
    state_dir: &Path,
    is_json: bool,
) -> anyhow::Result<()> {
    let id_to_undo_str = match operation_id {
        Some(id) => id,
        None => {
            // Find the most recent successful, non-undone operation
            let entries = history.get_history()?;
            let mut found = None;
            for entry in entries {
                if entry.undone_at.is_none() && entry.status == OperationStatus::Completed {
                    found = Some(entry.id.to_string());
                    break;
                }
            }
            match found {
                Some(id) => {
                    if !is_json {
                        println!(
                            "No operation ID provided, defaulting to last operation: {}",
                            id
                        );
                    }
                    id
                }
                None => {
                    anyhow::bail!("No eligible recent operations found to undo.");
                }
            }
        }
    };

    let id_to_undo = Uuid::parse_str(&id_to_undo_str)?;

    let result_path = state_dir.join(format!("result_{}.json", id_to_undo));
    if !result_path.exists() {
        anyhow::bail!("ExecutionResult not found for {}. The CLI currently requires the result_{{id}}.json file to perform an undo.", id_to_undo);
    }

    let result_json = std::fs::read_to_string(&result_path)?;
    let execution_result: ExecutionResult = serde_json::from_str(&result_json)?;

    if !is_json {
        println!("Undoing operation {}...", id_to_undo);
    }

    let files_restored = undo_execution(&execution_result)?;

    // Mark as undone in history
    let _ = history.mark_undone(id_to_undo);

    if is_json {
        println!(
            r#"{{"status": "Success", "files_restored": {}}}"#,
            files_restored
        );
    } else {
        println!("Successfully restored {} files.", files_restored);
    }

    Ok(())
}
