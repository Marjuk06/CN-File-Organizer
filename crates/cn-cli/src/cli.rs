use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "organize")]
#[command(about = "CN File Organizer - Smart local-first file organization", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Print output in JSON format
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the interactive Terminal UI (TUI)
    Tui,

    /// Scan a directory and show a summary of what's inside
    Scan {
        /// The path to scan
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Maximum depth to scan (default: unlimited)
        #[arg(short, long)]
        max_depth: Option<usize>,

        /// Do not skip hidden files
        #[arg(long)]
        include_hidden: bool,
    },

    /// Create an organization plan without executing it
    Plan {
        /// The path to organize
        #[arg(default_value = ".")]
        path: PathBuf,

        /// The mode to use (Smart, ByCategory, ByExtension, ByDate, BySize)
        #[arg(short, long, default_value = "Smart")]
        mode: String,

        /// Optional destination folder (if different from source)
        #[arg(short, long)]
        destination: Option<PathBuf>,
    },

    /// Organize files based on a plan
    Execute {
        /// The path to organize
        #[arg(default_value = ".")]
        path: PathBuf,

        /// The mode to use
        #[arg(short, long, default_value = "Smart")]
        mode: String,

        /// Automatically approve the plan and execute without prompting
        #[arg(short, long)]
        yes: bool,
    },

    /// Show history of operations
    History {
        /// Limit the number of entries shown
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Undo a previous operation
    Undo {
        /// The ID of the operation to undo (use 'history' to find IDs). If omitted, undoes the last operation.
        operation_id: Option<String>,
    },
}
