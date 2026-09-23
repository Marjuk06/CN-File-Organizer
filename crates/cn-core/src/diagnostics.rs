use crate::error::CnResult;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub os: String,
    pub arch: String,
    pub config_dir_exists: bool,
    pub state_dir_exists: bool,
    pub data_dir_exists: bool,
    pub app_version: String,
}

pub fn collect_diagnostics() -> CnResult<DiagnosticReport> {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();
    
    let config_dir = dirs::config_dir().map(|p| p.join("cn-file-organizer"));
    let state_dir = dirs::state_dir().map(|p| p.join("cn-file-organizer"));
    let data_dir = dirs::data_dir().map(|p| p.join("cn-file-organizer"));
    
    Ok(DiagnosticReport {
        os,
        arch,
        config_dir_exists: config_dir.map_or(false, |p| p.exists()),
        state_dir_exists: state_dir.map_or(false, |p| p.exists()),
        data_dir_exists: data_dir.map_or(false, |p| p.exists()),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
