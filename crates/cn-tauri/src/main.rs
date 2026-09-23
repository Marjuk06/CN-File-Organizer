// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use commands::{config::*, executor::*, history::*, planner::*, scanner::*};
use state::AppState;
use std::sync::Arc;
use tauri::Manager;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_updater::Builder::new().build()) // TODO: enable when updater is configured
        .setup(|app| {
            // Initialize AppState
            let state = AppState::new(app.handle())
                .map_err(|e| Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>)?;

            // Check for crash recovery
            let journal = cn_core::transaction::journal::TransactionJournal::new(&state.state_dir);
            if let Ok(recovered) = cn_core::transaction::recovery::RecoveryEngine::recover(&journal)
            {
                if recovered > 0 {
                    tracing::warn!(
                        "Recovered {} interrupted files from previous crash",
                        recovered
                    );
                }
            }

            app.manage(state);
            app.manage(Arc::new(ActiveOperations::default()));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Scanner
            scan_directory,
            // Planner
            create_plan,
            // Executor
            execute_plan_command,
            cancel_execution,
            // History
            get_history,
            clear_history,
            undo_operation,
            // Config
            get_settings,
            save_settings,
            get_rules,
            save_rules,
            get_diagnostics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
