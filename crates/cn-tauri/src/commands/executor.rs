use crate::state::AppState;
use cn_core::executor::{execute_plan, CancellationToken};
use cn_core::models::execution::ExecutionResult;
use cn_core::models::operation::OperationPlan;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

// Global state for cancellation tokens for active operations
pub struct ActiveOperations {
    tokens: Mutex<std::collections::HashMap<Uuid, CancellationToken>>,
}

impl Default for ActiveOperations {
    fn default() -> Self {
        Self {
            tokens: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[derive(serde::Serialize, Clone)]
struct ProgressPayload {
    event_type: String,
    operation_id: Option<Uuid>,
    source: Option<String>,
    destination: Option<String>,
    outcome: Option<String>,
    bytes_processed: Option<u64>,
    error: Option<String>,
}

#[tauri::command(async)]
pub async fn execute_plan_command(
    app: AppHandle,
    state: State<'_, AppState>,
    active_ops: State<'_, Arc<ActiveOperations>>,
    plan: OperationPlan,
) -> Result<ExecutionResult, String> {
    let operation_id = plan.id;
    let state_dir = state.state_dir.clone();
    let history_store = &state.history_store;

    let cancel_token = CancellationToken::new();

    // Register token
    {
        let mut tokens = active_ops.tokens.lock().await;
        tokens.insert(operation_id, cancel_token.clone());
    }

    let (progress_tx, mut progress_rx) = mpsc::channel(100);

    // Spawn a task to forward progress events to the frontend via Tauri events
    let app_clone = app.clone();
    tokio::spawn(async move {
        while let Some(event) = progress_rx.recv().await {
            use cn_core::executor::ExecuteProgressEvent::*;
            let payload = match event {
                Started {
                    operation_id,
                    source,
                    destination,
                } => ProgressPayload {
                    event_type: "started".into(),
                    operation_id: Some(operation_id),
                    source: Some(source),
                    destination: Some(destination),
                    outcome: None,
                    bytes_processed: None,
                    error: None,
                },
                Finished {
                    operation_id,
                    outcome,
                    bytes_processed,
                } => ProgressPayload {
                    event_type: "finished".into(),
                    operation_id: Some(operation_id),
                    source: None,
                    destination: None,
                    outcome: Some(format!("{:?}", outcome)),
                    bytes_processed: Some(bytes_processed),
                    error: None,
                },
                Complete => ProgressPayload {
                    event_type: "complete".into(),
                    operation_id: None,
                    source: None,
                    destination: None,
                    outcome: None,
                    bytes_processed: None,
                    error: None,
                },
                Error(e) => ProgressPayload {
                    event_type: "error".into(),
                    operation_id: None,
                    source: None,
                    destination: None,
                    outcome: None,
                    bytes_processed: None,
                    error: Some(e),
                },
            };

            let _ = app_clone.emit("execute-progress", payload);
        }
    });

    let plan_clone = plan.clone(); // Keep for history saving

    let res = execute_plan(plan, &state_dir, cancel_token, Some(progress_tx)).await;

    // Deregister token
    {
        let mut tokens = active_ops.tokens.lock().await;
        tokens.remove(&operation_id);
    }

    let result = res.map_err(|e| e.to_string())?;

    // Save to history
    if let Err(e) = history_store.add_entry(&result, &plan_clone) {
        tracing::error!("Failed to save history: {}", e);
    }

    Ok(result)
}

#[tauri::command]
pub async fn cancel_execution(
    active_ops: State<'_, Arc<ActiveOperations>>,
    operation_id: Uuid,
) -> Result<(), String> {
    let tokens = active_ops.tokens.lock().await;
    if let Some(token) = tokens.get(&operation_id) {
        token.cancel();
        Ok(())
    } else {
        Err("Operation not found or already completed".into())
    }
}
