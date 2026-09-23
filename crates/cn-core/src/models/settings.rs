use crate::models::operation::ConflictStrategy;
use serde::{Deserialize, Serialize};

/// Versioned user settings.
/// Stored as TOML in `~/.config/cn-file-organizer/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Schema version for migration support.
    pub schema_version: u32,
    pub general: GeneralSettings,
    pub conflict: ConflictSettings,
    pub safety: SafetySettings,
    pub appearance: AppearanceSettings,
    pub updates: UpdateSettings,
    pub advanced: AdvancedSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            schema_version: 1,
            general: GeneralSettings::default(),
            conflict: ConflictSettings::default(),
            safety: SafetySettings::default(),
            appearance: AppearanceSettings::default(),
            updates: UpdateSettings::default(),
            advanced: AdvancedSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Default organize mode name. e.g. "smart"
    pub default_mode: String,
    /// Show confirmation dialog before executing.
    pub confirm_before_execute: bool,
    /// Include hidden files in scans.
    pub include_hidden: bool,
    /// Destination mode: "in-place" or an absolute path.
    pub destination: String,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        GeneralSettings {
            default_mode: "smart".into(),
            confirm_before_execute: true,
            include_hidden: false,
            destination: "in-place".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictSettings {
    pub strategy: ConflictStrategy,
}

impl Default for ConflictSettings {
    fn default() -> Self {
        ConflictSettings {
            strategy: ConflictStrategy::Ask,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SafetySettings {
    pub follow_symlinks: bool,
    /// Additional user-defined protected paths.
    pub custom_protected_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    /// "system" | "light" | "dark"
    pub theme: String,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        AppearanceSettings {
            theme: "system".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettings {
    pub check_for_updates: bool,
    pub last_check: Option<String>,
}

impl Default for UpdateSettings {
    fn default() -> Self {
        UpdateSettings {
            check_for_updates: true,
            last_check: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    /// Number of scanner threads (0 = rayon auto).
    pub scanner_threads: usize,
    /// Log level: "error" | "warn" | "info" | "debug" | "trace"
    pub log_level: String,
    pub debug_mode: bool,
    /// Whether first-run setup has been completed.
    pub first_run_complete: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        AdvancedSettings {
            scanner_threads: 0,
            log_level: "info".into(),
            debug_mode: false,
            first_run_complete: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_conservative() {
        let s = Settings::default();
        assert!(!s.general.include_hidden);
        assert!(s.general.confirm_before_execute);
        assert!(!s.safety.follow_symlinks);
        assert_eq!(s.general.destination, "in-place");
    }

    #[test]
    fn default_conflict_strategy_is_ask() {
        let s = Settings::default();
        assert_eq!(s.conflict.strategy, ConflictStrategy::Ask);
    }

    #[test]
    fn default_theme_is_system() {
        let s = Settings::default();
        assert_eq!(s.appearance.theme, "system");
    }

    #[test]
    fn settings_round_trips_toml() {
        let s = Settings::default();
        let toml_str = toml::to_string(&s).unwrap();
        let s2: Settings = toml::from_str(&toml_str).unwrap();
        assert_eq!(s.schema_version, s2.schema_version);
        assert_eq!(s.general.default_mode, s2.general.default_mode);
    }
}
