mod cli;
mod commands;

use clap::Parser;
use cn_core::{
    history::HistoryStore,
    transaction::{TransactionJournal, RecoveryEngine},
};
use dirs::data_local_dir;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Setup logging unless JSON output is requested
    if !cli.json {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("cn_core=info".parse()?))
            .init();
    }

    // Determine paths
    let app_dir = get_app_data_dir();
    // Initialize core stores
    let history_store = HistoryStore::new(&app_dir);
    let journal = TransactionJournal::new(&app_dir);
    
    // Check for incomplete operations
    let recovered = RecoveryEngine::recover(&journal)?;
    if recovered > 0 {
        if !cli.json {
            println!("Recovered {} interrupted files.", recovered);
        }
    }

    match cli.command {
        Some(cli::Commands::Tui) => {
            commands::tui::run().await?;
        }
        Some(cli::Commands::Scan { path, max_depth, include_hidden }) => {
            commands::scan::run(path, max_depth, include_hidden, cli.json).await?;
        }
        Some(cli::Commands::Plan { path, mode, destination }) => {
            commands::plan::run(path, mode, destination, cli.json).await?;
        }
        Some(cli::Commands::Execute { path, mode, yes }) => {
            commands::execute::run(path, mode, yes, &app_dir, cli.json).await?;
        }
        Some(cli::Commands::History { limit }) => {
            commands::history::run(&history_store, limit, cli.json).await?;
        }
        Some(cli::Commands::Undo { operation_id }) => {
            commands::undo::run(operation_id, &history_store, &app_dir, cli.json).await?;
        }
        None => {
            // Default to TUI if no command is provided
            commands::tui::run().await?;
        }
    }

    Ok(())
}

fn get_app_data_dir() -> PathBuf {
    let mut dir = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("cn-organizer");
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    dir
}
