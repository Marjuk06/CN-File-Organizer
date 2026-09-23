pub mod journal;
pub mod recovery;

// Re-exports
pub use journal::{JournalEntry, TransactionJournal};
pub use recovery::RecoveryEngine;
