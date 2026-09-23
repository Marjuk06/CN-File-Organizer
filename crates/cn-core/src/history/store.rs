use crate::error::{CnError, CnResult};
use crate::models::execution::ExecutionResult;
use crate::models::history::{HistoryEntry, OperationStatus};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Manages the persistent operation history log.
#[derive(Clone)]
pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    /// Initialize a new history store in the state directory.
    pub fn new(state_dir: &Path) -> Self {
        let path = state_dir.join("history.jsonl");
        HistoryStore { path }
    }

    /// Add a new execution result to the history log.
    pub fn add_entry(&self, result: &ExecutionResult, plan: &crate::models::operation::OperationPlan) -> CnResult<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let entry = HistoryEntry {
            id: result.operation_id,
            timestamp: chrono::Utc::now(),
            source_dir: plan.source_dir.clone(),
            destination_dir: plan.destination_dir.clone(),
            mode: format!("{:?}", plan.mode),
            file_count: result.total_files(),
            total_bytes: plan.estimated_bytes,
            status: match result.status {
                crate::models::execution::ExecutionStatus::Completed => OperationStatus::Completed,
                crate::models::execution::ExecutionStatus::PartialSuccess => OperationStatus::PartialSuccess,
                crate::models::execution::ExecutionStatus::Failed => OperationStatus::Failed,
                crate::models::execution::ExecutionStatus::Cancelled => OperationStatus::Cancelled,
            },
            undone_at: None,
        };

        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let json = serde_json::to_string(&entry)?;
        writeln!(f, "{}", json)?;
        
        Ok(())
    }

    /// Read all history entries, sorted newest first.
    pub fn get_history(&self) -> CnResult<Vec<HistoryEntry>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let f = File::open(&self.path)?;
        let reader = BufReader::new(f);
        let mut entries: Vec<HistoryEntry> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(entry) = serde_json::from_str(&line) {
                entries.push(entry);
            }
        }
        
        // Sort newest first
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(entries)
    }

    /// Mark a specific operation as undone.
    pub fn mark_undone(&self, id: Uuid) -> CnResult<()> {
        let mut entries = self.get_history()?;
        let mut found = false;
        
        for entry in &mut entries {
            if entry.id == id {
                entry.undone_at = Some(chrono::Utc::now());
                entry.status = OperationStatus::Undone;
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(CnError::OperationNotFound(id.to_string()));
        }
        
        // Re-write the whole file (history is kept small enough to allow this)
        // Sort oldest first for chronological append
        entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
            
        for entry in entries {
            let json = serde_json::to_string(&entry)?;
            writeln!(f, "{}", json)?;
        }
        
        Ok(())
    }
    
    /// Clear all history
    pub fn clear(&self) -> CnResult<()> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}
