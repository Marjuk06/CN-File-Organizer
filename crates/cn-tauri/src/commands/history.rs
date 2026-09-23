use crate::state::AppState;
use cn_core::models::history::HistoryEntry;
use cn_core::undo::undo_execution;
use cn_core::models::execution::ExecutionResult;
use tauri::State;


#[tauri::command(async)]
pub async fn get_history(state: State<'_, AppState>) -> Result<Vec<HistoryEntry>, String> {
    let store = state.history_store.clone();
    tokio::task::spawn_blocking(move || {
        store.get_history().map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command(async)]
pub async fn clear_history(state: State<'_, AppState>) -> Result<(), String> {
    let store = state.history_store.clone();
    tokio::task::spawn_blocking(move || {
        store.clear().map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

// In a real app we'd need to store the full ExecutionResult somewhere to undo it.
// We are skipping the full serialization of ExecutionResult in history for this prototype,
// so undoing requires the result object.
#[tauri::command(async)]
pub async fn undo_operation(
    state: State<'_, AppState>,
    result: ExecutionResult,
) -> Result<u64, String> {
    let operation_id = result.operation_id;
    let res = tokio::task::spawn_blocking(move || {
        undo_execution(&result).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;
    
    // Mark as undone
    let _ = state.history_store.mark_undone(operation_id);
    
    Ok(res)
}
