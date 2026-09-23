use crate::error::{CnError, CnResult};
use crate::models::rule::Rule;
use crate::models::settings::Settings;
use std::fs;
use std::path::{Path, PathBuf};

/// Manages loading and saving configuration (settings + rules).
#[derive(Clone)]
pub struct ConfigStore {
    config_dir: PathBuf,
}

impl ConfigStore {
    /// Initialize with a specific config directory.
    pub fn new(config_dir: &Path) -> Self {
        ConfigStore {
            config_dir: config_dir.to_path_buf(),
        }
    }

    /// Default initialization using XDG base directories.
    pub fn default_store() -> CnResult<Self> {
        let dir = dirs::config_dir()
            .ok_or_else(|| CnError::Config("Could not determine user config directory".into()))?
            .join("cn-file-organizer");
        Ok(Self::new(&dir))
    }

    pub fn settings_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    pub fn rules_path(&self) -> PathBuf {
        self.config_dir.join("rules.json")
    }

    pub fn load_settings(&self) -> CnResult<Settings> {
        let path = self.settings_path();
        if !path.exists() {
            return Ok(Settings::default());
        }

        let content = fs::read_to_string(&path)?;
        let settings: Settings = toml::from_str(&content)?;

        // Very basic schema version check
        let current_schema = Settings::default().schema_version;
        if settings.schema_version > current_schema {
            return Err(CnError::ConfigVersionTooNew {
                found: settings.schema_version,
                supported: current_schema,
            });
        }

        Ok(settings)
    }

    pub fn save_settings(&self, settings: &Settings) -> CnResult<()> {
        fs::create_dir_all(&self.config_dir)?;
        let content = toml::to_string_pretty(settings)?;
        fs::write(self.settings_path(), content)?;
        Ok(())
    }

    pub fn load_rules(&self) -> CnResult<Vec<Rule>> {
        let path = self.rules_path();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)?;
        let rules: Vec<Rule> = serde_json::from_str(&content)?;
        Ok(rules)
    }

    pub fn save_rules(&self, rules: &[Rule]) -> CnResult<()> {
        fs::create_dir_all(&self.config_dir)?;
        let content = serde_json::to_string_pretty(rules)?;
        fs::write(self.rules_path(), content)?;
        Ok(())
    }
}
