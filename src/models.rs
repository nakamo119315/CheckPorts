//! Data models for the ports CLI application.
//!
//! This module defines the core data structures used to represent
//! port entries, process information, and application types.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::Duration;

/// Network protocol type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Protocol {
    /// TCP protocol
    Tcp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
        }
    }
}

/// Application type detected from the command line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AppType {
    /// Node.js application (node, npm, yarn)
    NodeJs,
    /// Python application (python, uvicorn, gunicorn)
    Python,
    /// .NET application (dotnet, *.dll)
    DotNet,
    /// Java application (java, *.jar)
    Java,
    /// Go application (go, gin, echo)
    Go,
    /// Ruby application (ruby, rails, puma)
    Ruby,
    /// PHP application (php, artisan)
    Php,
    /// Rust application (cargo, target/)
    Rust,
    /// Nginx web server
    Nginx,
    /// Apache web server (httpd, apache)
    Apache,
    /// Unknown application type
    Unknown,
}

impl AppType {
    /// Returns the display name for this application type.
    pub fn display_name(&self) -> &'static str {
        match self {
            AppType::NodeJs => "Node.js",
            AppType::Python => "Python",
            AppType::DotNet => ".NET",
            AppType::Java => "Java",
            AppType::Go => "Go",
            AppType::Ruby => "Ruby",
            AppType::Php => "PHP",
            AppType::Rust => "Rust",
            AppType::Nginx => "Nginx",
            AppType::Apache => "Apache",
            AppType::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Information about a process using a network port.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process name (executable name)
    pub name: String,
    /// Full command line (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Process start time (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    /// Time elapsed since process started
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_duration")]
    pub elapsed: Option<Duration>,
    /// User running the process (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ProcessInfo {
    /// Creates a new ProcessInfo with basic information.
    pub fn new(pid: u32, name: impl Into<String>) -> Self {
        Self {
            pid,
            name: name.into(),
            command: None,
            started_at: None,
            elapsed: None,
            user: None,
        }
    }

    /// Sets the command line.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Sets the start time and calculates elapsed duration.
    pub fn with_started_at(mut self, started_at: DateTime<Utc>) -> Self {
        self.started_at = Some(started_at);
        let now = Utc::now();
        if started_at <= now {
            let elapsed_seconds = (now - started_at).num_seconds();
            if elapsed_seconds >= 0 {
                self.elapsed = Some(Duration::from_secs(elapsed_seconds as u64));
            }
        }
        self
    }

    /// Sets the user.
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Returns a human-readable elapsed time string.
    pub fn elapsed_human(&self) -> Option<String> {
        self.elapsed.map(format_duration)
    }
}

/// A listening port entry with associated process information.
#[derive(Debug, Clone, Serialize)]
pub struct PortEntry {
    /// The port number (1-65535)
    pub port: u16,
    /// The network protocol (TCP)
    pub protocol: Protocol,
    /// Information about the process using this port
    pub process: ProcessInfo,
    /// Detected application type (if identifiable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<AppType>,
}

impl PortEntry {
    /// Creates a new PortEntry.
    pub fn new(port: u16, protocol: Protocol, process: ProcessInfo) -> Self {
        Self {
            port,
            protocol,
            process,
            app_type: None,
        }
    }

    /// Sets the application type.
    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.app_type = Some(app_type);
        self
    }
}

/// Formats a Duration into a human-readable string (e.g., "2h 15m", "1d 3h").
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    if total_seconds < 60 {
        return format!("{}s", total_seconds);
    }

    let minutes = total_seconds / 60;
    if minutes < 60 {
        return format!("{}m", minutes);
    }

    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    if hours < 24 {
        if remaining_minutes > 0 {
            return format!("{}h {}m", hours, remaining_minutes);
        }
        return format!("{}h", hours);
    }

    let days = hours / 24;
    let remaining_hours = hours % 24;
    if remaining_hours > 0 {
        format!("{}d {}h", days, remaining_hours)
    } else {
        format!("{}d", days)
    }
}

/// Custom serializer for Duration to output seconds.
fn serialize_duration<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match duration {
        Some(d) => serializer.serialize_u64(d.as_secs()),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(Duration::from_secs(90)), "1m");
        assert_eq!(format_duration(Duration::from_secs(300)), "5m");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h");
        assert_eq!(format_duration(Duration::from_secs(3600 + 900)), "1h 15m");
    }

    #[test]
    fn test_format_duration_days() {
        assert_eq!(format_duration(Duration::from_secs(86400)), "1d");
        assert_eq!(format_duration(Duration::from_secs(86400 + 10800)), "1d 3h");
    }

    #[test]
    fn test_app_type_display() {
        assert_eq!(AppType::NodeJs.display_name(), "Node.js");
        assert_eq!(AppType::DotNet.display_name(), ".NET");
        assert_eq!(AppType::Unknown.display_name(), "Unknown");
    }
}
