//! cn-core — Pure Rust core engine for CN File Organizer
//!
//! This crate is completely independent of Tauri, React, and any UI framework.
//! It is consumed by both the desktop GUI (cn-tauri) and the CLI (cn-cli).

pub mod config;
pub mod classifier;
pub mod diagnostics;
pub mod duplicate;
pub mod error;
pub mod executor;
pub mod history;
pub mod models;
pub mod planner;
pub mod rules;
pub mod safety;
pub mod scanner;
pub mod transaction;
pub mod undo;

// Convenience re-exports of the most commonly used public types
pub use error::{CnError, CnResult};
pub use models::{
    category::Category,
    conflict::ConflictInfo,
    execution::{ExecutionResult, FileResult},
    file_info::{FileInfo, FileKind},
    history::{HistoryEntry, OperationStatus},
    operation::{FileOperation, OperationKind, OperationPlan, OrganizeMode},
    rule::{ConditionMode, Rule, RuleAction, RuleCondition},
    scan_summary::ScanSummary,
    settings::Settings,
};
