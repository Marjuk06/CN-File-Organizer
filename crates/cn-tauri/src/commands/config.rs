use crate::state::AppState;
use cn_core::diagnostics::{collect_diagnostics, DiagnosticReport};
use cn_core::models::rule::Rule;
use cn_core::models::settings::Settings;
use tauri::State;

#[tauri::command(async)]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let store = state.config_store.clone();
    tokio::task::spawn_blocking(move || store.load_settings().map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command(async)]
pub async fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    let store = state.config_store.clone();
    tokio::task::spawn_blocking(move || {
        store.save_settings(&settings).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command(async)]
pub async fn get_rules(state: State<'_, AppState>) -> Result<Vec<Rule>, String> {
    let store = state.config_store.clone();
    tokio::task::spawn_blocking(move || store.load_rules().map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command(async)]
pub async fn save_rules(state: State<'_, AppState>, rules: Vec<Rule>) -> Result<(), String> {
    // Validate rules before saving
    for rule in &rules {
        if let Err(e) = cn_core::rules::validator::validate_rule(rule) {
            return Err(e.to_string());
        }
    }
    
    let store = state.config_store.clone();
    tokio::task::spawn_blocking(move || {
        store.save_rules(&rules).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command(async)]
pub async fn get_diagnostics() -> Result<DiagnosticReport, String> {
    tokio::task::spawn_blocking(move || collect_diagnostics().map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}
