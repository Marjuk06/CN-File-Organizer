use std::path::PathBuf;
use cn_core::config::ConfigStore;
use cn_core::history::HistoryStore;
use tauri::{AppHandle, Manager};

pub struct AppState {
    pub config_store: ConfigStore,
    pub history_store: HistoryStore,
    pub state_dir: PathBuf,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let path_resolver = app.path();
        
        let config_dir = path_resolver
            .app_config_dir()
            .map_err(|_| "Could not determine config directory".to_string())?;
            
        let state_dir = path_resolver
            .app_local_data_dir()
            .map_err(|_| "Could not determine state directory".to_string())?;

        // Ensure directories exist
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&state_dir).map_err(|e| e.to_string())?;

        Ok(AppState {
            config_store: ConfigStore::new(&config_dir),
            history_store: HistoryStore::new(&state_dir),
            state_dir,
        })
    }
}
