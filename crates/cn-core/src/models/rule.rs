use crate::models::category::Category;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A custom organization rule defined by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: Uuid,
    /// Human-readable name for display.
    pub name: String,
    /// Whether this rule is currently active.
    pub enabled: bool,
    /// Evaluation priority. Lower numbers are evaluated first.
    /// The first matching rule wins.
    pub priority: u32,
    /// All conditions that must be evaluated.
    pub conditions: Vec<RuleCondition>,
    /// How multiple conditions are combined.
    pub condition_mode: ConditionMode,
    /// What to do when this rule matches.
    pub action: RuleAction,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Rule {
    pub fn new(name: impl Into<String>, action: RuleAction) -> Self {
        let now = Utc::now();
        Rule {
            id: Uuid::new_v4(),
            name: name.into(),
            enabled: true,
            priority: 100,
            conditions: Vec::new(),
            condition_mode: ConditionMode::All,
            action,
            created_at: now,
            modified_at: now,
        }
    }
}

/// A condition that a file must satisfy for a rule to match.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RuleCondition {
    /// File extension (without dot, case-insensitive). e.g. "pdf"
    Extension { value: String },
    /// MIME type string. e.g. "image/png"
    MimeType { value: String },
    /// File belongs to a category.
    Category { value: Category },
    /// Filename matches a glob pattern. e.g. "invoice_*"
    NamePattern { pattern: String },
    /// File size is greater than N bytes.
    SizeGreaterThan { bytes: u64 },
    /// File size is less than N bytes.
    SizeLessThan { bytes: u64 },
    /// File was modified before this datetime.
    ModifiedBefore { datetime: DateTime<Utc> },
    /// File was modified after this datetime.
    ModifiedAfter { datetime: DateTime<Utc> },
}

/// How to combine multiple conditions in a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionMode {
    /// All conditions must match.
    All,
    /// At least one condition must match.
    Any,
}

/// What action to take when a rule matches.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RuleAction {
    /// Move to a subfolder relative to the destination directory.
    MoveToSubfolder { folder: String },
    /// Skip this file; do not organize it.
    Skip,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rule_is_enabled_by_default() {
        let r = Rule::new("test", RuleAction::Skip);
        assert!(r.enabled);
    }

    #[test]
    fn condition_mode_serializes() {
        let json = serde_json::to_string(&ConditionMode::Any).unwrap();
        assert_eq!(json, r#""any""#);
    }

    #[test]
    fn rule_action_skip_serializes() {
        let json = serde_json::to_string(&RuleAction::Skip).unwrap();
        assert!(json.contains("skip"));
    }

    #[test]
    fn rule_condition_extension_serializes() {
        let cond = RuleCondition::Extension {
            value: "pdf".into(),
        };
        let json = serde_json::to_string(&cond).unwrap();
        assert!(json.contains("extension"));
        assert!(json.contains("pdf"));
    }
}
