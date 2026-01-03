//! Output formatting for port entries.
//!
//! This module provides functions to format port entries as either
//! human-readable tables or machine-readable JSON.

use crate::models::{format_duration, PortEntry};
use chrono::Utc;
use serde::Serialize;

/// Output format selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable table format
    Table,
    /// Machine-readable JSON format
    Json,
}

/// JSON output structure.
#[derive(Serialize)]
struct JsonOutput<'a> {
    ports: &'a [PortEntry],
    total_count: usize,
    timestamp: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
}

/// Prints port entries in the specified format.
pub fn print_entries(entries: &[PortEntry], format: OutputFormat) {
    match format {
        OutputFormat::Table => print_table(entries),
        OutputFormat::Json => print_json(entries),
    }
}

/// Prints port entries as a formatted table.
fn print_table(entries: &[PortEntry]) {
    if entries.is_empty() {
        println!("アクティブなポートはありません");
        return;
    }

    // Print header
    println!(
        "{:>5}  {:>6}  {:<8}  {:>8}  {}",
        "PORT", "PID", "TYPE", "UPTIME", "COMMAND"
    );

    // Print entries
    for entry in entries {
        let app_type = entry
            .app_type
            .as_ref()
            .map(|t| t.display_name())
            .unwrap_or("Unknown");

        let uptime = entry
            .process
            .elapsed
            .map(format_duration)
            .unwrap_or_else(|| "-".to_string());

        let command = entry
            .process
            .command
            .as_deref()
            .unwrap_or(&entry.process.name);

        // Truncate command if too long for display
        let command_display = if command.len() > 60 {
            format!("{}...", &command[..57])
        } else {
            command.to_string()
        };

        println!(
            "{:>5}  {:>6}  {:<8}  {:>8}  {}",
            entry.port, entry.process.pid, app_type, uptime, command_display
        );
    }
}

/// Prints port entries as JSON.
fn print_json(entries: &[PortEntry]) {
    let output = JsonOutput {
        ports: entries,
        total_count: entries.len(),
        timestamp: Utc::now().to_rfc3339(),
        warnings: Vec::new(),
    };

    match serde_json::to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("error: Failed to serialize JSON: {}", e),
    }
}

/// Prints a warning message (used for partial information retrieval).
pub fn print_warning(message: &str) {
    eprintln!("注意: {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppType, ProcessInfo, Protocol};

    fn make_entry(port: u16, pid: u32, name: &str) -> PortEntry {
        let process = ProcessInfo::new(pid, name);
        PortEntry::new(port, Protocol::Tcp, process)
    }

    #[test]
    fn test_print_table_empty() {
        // This test just ensures the function doesn't panic
        print_table(&[]);
    }

    #[test]
    fn test_print_table_with_entries() {
        let mut entry = make_entry(3000, 1234, "node");
        entry.app_type = Some(AppType::NodeJs);
        entry.process.command = Some("node server.js".to_string());
        entry.process.elapsed = Some(std::time::Duration::from_secs(3600));

        // This test just ensures the function doesn't panic
        print_table(&[entry]);
    }
}
