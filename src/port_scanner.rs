//! Port scanning functionality.
//!
//! This module provides functions to scan for listening TCP ports on macOS.

use crate::error::{PortsError, Result};
use crate::models::{PortEntry, ProcessInfo, Protocol};
use std::process::Command;

/// Scans for all listening TCP ports and returns port entries.
///
/// Uses the `lsof` command to find listening TCP sockets and their
/// associated processes. This approach is reliable on macOS and
/// works with standard user permissions.
pub fn scan_listening_ports() -> Result<Vec<PortEntry>> {
    // Use lsof to get listening TCP ports
    // -iTCP: Select TCP connections
    // -sTCP:LISTEN: Only show LISTEN state
    // -n: Don't resolve hostnames (faster)
    // -P: Don't resolve port names (show numbers)
    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-n", "-P", "-F", "pcn"])
        .output()
        .map_err(|e| PortsError::system_error(format!("Failed to execute lsof: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PortsError::system_error(format!(
            "lsof failed: {}",
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_lsof_output(&stdout)
}

/// Parses lsof -F output format.
///
/// Format uses single-letter prefixes:
/// - p: PID
/// - c: Command name
/// - n: Name (includes port info like *:3000 or 127.0.0.1:8080)
fn parse_lsof_output(output: &str) -> Result<Vec<PortEntry>> {
    let mut entries = Vec::new();
    let mut current_pid: Option<u32> = None;
    let mut current_name: Option<String> = None;

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }

        let prefix = line.chars().next().unwrap_or(' ');
        let value = &line[1..];

        match prefix {
            'p' => {
                current_pid = value.parse().ok();
            }
            'c' => {
                current_name = Some(value.to_string());
            }
            'n' => {
                // Parse port from name like "*:3000" or "127.0.0.1:8080"
                if let Some(port) = extract_port_from_name(value) {
                    if let (Some(pid), Some(name)) = (current_pid, current_name.as_ref()) {
                        let process = ProcessInfo::new(pid, name.clone());
                        let entry = PortEntry::new(port, Protocol::Tcp, process);

                        // Avoid duplicates (same port, same PID)
                        if !entries.iter().any(|e: &PortEntry| e.port == port && e.process.pid == pid) {
                            entries.push(entry);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(entries)
}

/// Extracts the port number from an lsof name field.
///
/// Examples:
/// - "*:3000" -> Some(3000)
/// - "127.0.0.1:8080" -> Some(8080)
/// - "[::1]:9000" -> Some(9000)
fn extract_port_from_name(name: &str) -> Option<u16> {
    // Find the last colon and parse the port after it
    let port_str = name.rsplit(':').next()?;
    port_str.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_port_from_wildcard() {
        assert_eq!(extract_port_from_name("*:3000"), Some(3000));
    }

    #[test]
    fn test_extract_port_from_ipv4() {
        assert_eq!(extract_port_from_name("127.0.0.1:8080"), Some(8080));
    }

    #[test]
    fn test_extract_port_from_ipv6() {
        assert_eq!(extract_port_from_name("[::1]:9000"), Some(9000));
    }

    #[test]
    fn test_parse_lsof_output() {
        let output = "p1234\ncnode\nn*:3000\np5678\ncpython\nn127.0.0.1:8080\n";
        let entries = parse_lsof_output(output).unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].port, 3000);
        assert_eq!(entries[0].process.pid, 1234);
        assert_eq!(entries[0].process.name, "node");
        assert_eq!(entries[1].port, 8080);
        assert_eq!(entries[1].process.pid, 5678);
        assert_eq!(entries[1].process.name, "python");
    }
}
