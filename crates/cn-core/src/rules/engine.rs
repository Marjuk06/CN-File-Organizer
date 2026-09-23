use crate::models::file_info::FileInfo;
use crate::models::operation::{FileOperation, OperationKind};
use crate::models::rule::{ConditionMode, Rule, RuleAction, RuleCondition};
use std::path::Path;
use uuid::Uuid;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(mut rules: Vec<Rule>) -> Self {
        // Sort by priority (lowest first)
        rules.sort_by_key(|a| a.priority);
        Self { rules }
    }

    pub fn evaluate(&self, file: &FileInfo, destination_dir: &Path) -> Option<FileOperation> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            let matches = match rule.condition_mode {
                ConditionMode::All => rule.conditions.iter().all(|c| self.eval_condition(c, file)),
                ConditionMode::Any => rule.conditions.iter().any(|c| self.eval_condition(c, file)),
            };

            if matches {
                let kind = match &rule.action {
                    RuleAction::Skip => OperationKind::Skip,
                    RuleAction::MoveToSubfolder { .. } => OperationKind::Move,
                };

                let destination = match &rule.action {
                    RuleAction::Skip => {
                        destination_dir.join(file.path.file_name().unwrap_or_default())
                    }
                    RuleAction::MoveToSubfolder { folder } => destination_dir
                        .join(folder)
                        .join(file.path.file_name().unwrap_or_default()),
                };

                return Some(FileOperation {
                    id: Uuid::new_v4(),
                    source: file.path.clone(),
                    destination,
                    kind,
                    category: file.category,
                    size: file.size,
                    conflict: None,
                });
            }
        }

        // No rules matched
        None
    }

    fn eval_condition(&self, condition: &RuleCondition, file: &FileInfo) -> bool {
        match condition {
            RuleCondition::Extension { value } => {
                if let Some(ext) = &file.extension {
                    ext.eq_ignore_ascii_case(value)
                } else {
                    false
                }
            }
            RuleCondition::MimeType { value } => {
                if let Some(mime) = &file.mime_type {
                    mime.eq_ignore_ascii_case(value)
                } else {
                    false
                }
            }
            RuleCondition::Category { value } => {
                if let Some(cat) = &file.category {
                    cat == value
                } else {
                    false
                }
            }
            RuleCondition::NamePattern { pattern } => {
                let name = file.path.file_name().unwrap_or_default().to_string_lossy();
                // Extremely simple wildcard match for now (*pattern*)
                let p = pattern.trim_matches('*');
                name.contains(p)
            }
            RuleCondition::SizeGreaterThan { bytes } => file.size > *bytes,
            RuleCondition::SizeLessThan { bytes } => file.size < *bytes,
            RuleCondition::ModifiedBefore { datetime } => {
                if let Some(modified) = file.modified_at {
                    modified < *datetime
                } else {
                    false
                }
            }
            RuleCondition::ModifiedAfter { datetime } => {
                if let Some(modified) = file.modified_at {
                    modified > *datetime
                } else {
                    false
                }
            }
        }
    }
}
