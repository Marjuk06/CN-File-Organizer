use crate::error::{CnError, CnResult};
use crate::models::rule::Rule;

pub fn validate_rule(rule: &Rule) -> CnResult<()> {
    if rule.name.trim().is_empty() {
        return Err(CnError::InvalidRule("Rule name cannot be empty".into()));
    }

    if rule.conditions.is_empty() {
        return Err(CnError::InvalidRule(
            "Rule must have at least one condition".into(),
        ));
    }

    // Check for protected paths in MoveToSubfolder
    if let crate::models::rule::RuleAction::MoveToSubfolder { folder } = &rule.action {
        if folder.trim().is_empty() {
            return Err(CnError::InvalidRule(
                "Subfolder name cannot be empty".into(),
            ));
        }

        if folder.contains("..") || folder.starts_with('/') {
            return Err(CnError::InvalidRule(
                "Subfolder must be a relative path without parent traversals (..)".into(),
            ));
        }
    }

    Ok(())
}
