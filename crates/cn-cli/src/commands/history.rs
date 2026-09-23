use cn_core::history::HistoryStore;

pub async fn run(history: &HistoryStore, limit: usize, is_json: bool) -> anyhow::Result<()> {
    let mut entries = history.get_history()?;
    entries.truncate(limit);

    if is_json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else {
        if entries.is_empty() {
            println!("No history found.");
            return Ok(());
        }

        println!("\nOperation History (Last {}):", entries.len());
        println!("{:<36} | {:<20} | {:<15} | {:<10} | {:<12}", "ID", "Timestamp", "Mode", "Files", "Status");
        println!("{:-<103}", "");

        for entry in entries {
            let status = if entry.undone_at.is_some() {
                "Undone".to_string()
            } else {
                format!("{:?}", entry.status)
            };

            let timestamp = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{:<36} | {:<20} | {:<15} | {:<10} | {:<12}",
                entry.id,
                timestamp,
                entry.mode,
                entry.file_count,
                status
            );
        }
        
        println!("\nRun `organize undo <ID>` to rollback an operation.");
    }

    Ok(())
}
