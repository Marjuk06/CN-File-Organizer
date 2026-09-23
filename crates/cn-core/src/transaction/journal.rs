use crate::error::{CnError, CnResult};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// An entry in the write-ahead journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JournalEntry {
    /// Operation plan is starting.
    Start {
        operation_id: Uuid,
        plan_path: PathBuf,
    },
    /// A single file move has begun (but not yet finished).
    FileMoving {
        operation_id: Uuid,
        source: PathBuf,
        destination: PathBuf,
    },
    /// A single file move completed successfully.
    FileMoved {
        operation_id: Uuid,
        source: PathBuf,
        destination: PathBuf,
    },
    /// The entire operation finished successfully.
    Success { operation_id: Uuid },
    /// The operation failed or was cancelled.
    Failed { operation_id: Uuid, reason: String },
}

/// The write-ahead journal for tracking in-flight operations to allow crash recovery.
pub struct TransactionJournal {
    path: PathBuf,
    file: Option<File>,
}

impl TransactionJournal {
    /// Initialize a new journal in the state directory.
    pub fn new(state_dir: &Path) -> Self {
        let path = state_dir.join("journal.jsonl");
        TransactionJournal { path, file: None }
    }

    /// Open the journal file for writing.
    pub fn open(&mut self) -> CnResult<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        self.file = Some(f);
        Ok(())
    }

    /// Append an entry to the journal and fsync.
    pub fn append(&mut self, entry: &JournalEntry) -> CnResult<()> {
        if let Some(f) = &mut self.file {
            let json = serde_json::to_string(entry)?;
            writeln!(f, "{}", json)?;
            f.sync_data()?;
            Ok(())
        } else {
            Err(CnError::Journal("Journal not open".into()))
        }
    }

    /// Read all entries from the journal (used during recovery).
    pub fn read_all(&self) -> CnResult<Vec<JournalEntry>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let f = File::open(&self.path)?;
        let reader = BufReader::new(f);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(entry) = serde_json::from_str(&line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Clear the journal (usually done after a successful start-to-finish run or recovery).
    pub fn clear(&mut self) -> CnResult<()> {
        // Close current handle
        self.file = None;
        // Truncate file
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        self.file = Some(f);
        Ok(())
    }
}
