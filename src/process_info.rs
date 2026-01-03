//! Process information retrieval.
//!
//! This module provides functions to get detailed process information
//! on macOS using system commands and the libproc library.

use crate::error::{PortsError, Result};
use crate::models::ProcessInfo;
use chrono::{DateTime, TimeZone, Utc};
use std::process::Command;

/// Enriches a ProcessInfo with additional details from the system.
///
/// Attempts to retrieve:
/// - Full command line
/// - Process start time
/// - Running user
///
/// If some information cannot be retrieved (e.g., due to permissions),
/// the function will still succeed with partial information.
pub fn enrich_process_info(process: &mut ProcessInfo) -> Result<()> {
    // Get command line using ps
    if let Ok(command) = get_command_line(process.pid) {
        process.command = Some(command);
    }

    // Get process start time using ps
    if let Ok(started_at) = get_start_time(process.pid) {
        let now = Utc::now();
        if started_at <= now {
            let elapsed_seconds = (now - started_at).num_seconds();
            if elapsed_seconds >= 0 {
                process.elapsed = Some(std::time::Duration::from_secs(elapsed_seconds as u64));
            }
        }
        process.started_at = Some(started_at);
    }

    // Get user using ps
    if let Ok(user) = get_user(process.pid) {
        process.user = Some(user);
    }

    Ok(())
}

/// Gets the full command line for a process.
fn get_command_line(pid: u32) -> Result<String> {
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
        .map_err(|e| PortsError::system_error(format!("Failed to execute ps: {}", e)))?;

    if !output.status.success() {
        return Err(PortsError::process_not_found(pid));
    }

    let command = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if command.is_empty() {
        return Err(PortsError::process_not_found(pid));
    }

    Ok(command)
}

/// Gets the start time for a process.
fn get_start_time(pid: u32) -> Result<DateTime<Utc>> {
    // Use ps with lstart format for full start time
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "lstart="])
        .output()
        .map_err(|e| PortsError::system_error(format!("Failed to execute ps: {}", e)))?;

    if !output.status.success() {
        return Err(PortsError::process_not_found(pid));
    }

    let lstart = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if lstart.is_empty() {
        return Err(PortsError::process_not_found(pid));
    }

    // Parse lstart format: "Mon Jan  2 15:04:05 2006"
    parse_lstart(&lstart)
}

/// Parses the lstart format from ps.
///
/// Format: "Mon Jan  2 15:04:05 2006" (day of week, month, day, time, year)
fn parse_lstart(lstart: &str) -> Result<DateTime<Utc>> {
    // Try parsing with chrono
    // The format is: "Wed Jan  1 12:34:56 2025"
    let parts: Vec<&str> = lstart.split_whitespace().collect();
    if parts.len() < 5 {
        return Err(PortsError::system_error(format!(
            "Invalid lstart format: {}",
            lstart
        )));
    }

    // parts: [weekday, month, day, time, year]
    let month = match parts[1] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => {
            return Err(PortsError::system_error(format!(
                "Unknown month: {}",
                parts[1]
            )))
        }
    };

    let day: u32 = parts[2]
        .parse()
        .map_err(|_| PortsError::system_error(format!("Invalid day: {}", parts[2])))?;

    let time_parts: Vec<&str> = parts[3].split(':').collect();
    if time_parts.len() != 3 {
        return Err(PortsError::system_error(format!(
            "Invalid time format: {}",
            parts[3]
        )));
    }

    let hour: u32 = time_parts[0]
        .parse()
        .map_err(|_| PortsError::system_error(format!("Invalid hour: {}", time_parts[0])))?;
    let minute: u32 = time_parts[1]
        .parse()
        .map_err(|_| PortsError::system_error(format!("Invalid minute: {}", time_parts[1])))?;
    let second: u32 = time_parts[2]
        .parse()
        .map_err(|_| PortsError::system_error(format!("Invalid second: {}", time_parts[2])))?;

    let year: i32 = parts[4]
        .parse()
        .map_err(|_| PortsError::system_error(format!("Invalid year: {}", parts[4])))?;

    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .ok_or_else(|| PortsError::system_error(format!("Invalid date: {}", lstart)))
}

/// Gets the user running a process.
fn get_user(pid: u32) -> Result<String> {
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "user="])
        .output()
        .map_err(|e| PortsError::system_error(format!("Failed to execute ps: {}", e)))?;

    if !output.status.success() {
        return Err(PortsError::process_not_found(pid));
    }

    let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if user.is_empty() {
        return Err(PortsError::process_not_found(pid));
    }

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_lstart() {
        let lstart = "Wed Jan  1 12:34:56 2025";
        let result = parse_lstart(lstart);
        assert!(result.is_ok());

        let dt = result.unwrap();
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
        assert_eq!(dt.year(), 2025);
    }
}
