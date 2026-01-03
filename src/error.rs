//! Error types for the ports CLI application.
//!
//! This module defines all error types that can occur during port scanning
//! and process information retrieval.

use thiserror::Error;

/// The main error type for the ports application.
#[derive(Error, Debug)]
pub enum PortsError {
    /// Permission denied when accessing process information.
    /// This typically occurs when trying to read details of processes
    /// owned by other users without elevated privileges.
    #[error("Permission denied: {message}")]
    PermissionDenied {
        message: String,
    },

    /// The process was not found, likely because it terminated
    /// between the port scan and the detailed info retrieval.
    #[error("Process not found: PID {pid}")]
    ProcessNotFound {
        pid: u32,
    },

    /// A system-level error occurred while interacting with OS APIs.
    #[error("System error: {message}")]
    SystemError {
        message: String,
    },

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl PortsError {
    /// Creates a new PermissionDenied error with the given message.
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::PermissionDenied {
            message: message.into(),
        }
    }

    /// Creates a new ProcessNotFound error for the given PID.
    pub fn process_not_found(pid: u32) -> Self {
        Self::ProcessNotFound { pid }
    }

    /// Creates a new SystemError with the given message.
    pub fn system_error(message: impl Into<String>) -> Self {
        Self::SystemError {
            message: message.into(),
        }
    }

    /// Returns a hint for how to resolve this error.
    pub fn hint(&self) -> &'static str {
        match self {
            Self::PermissionDenied { .. } => {
                "Try running with elevated privileges (sudo) to see all process details"
            }
            Self::ProcessNotFound { .. } => {
                "The process may have terminated. Try running the command again"
            }
            Self::SystemError { .. } => {
                "Check system permissions and ensure the OS APIs are accessible"
            }
            Self::IoError(_) => {
                "Check file permissions and system resources"
            }
        }
    }
}

/// A type alias for Results with PortsError.
pub type Result<T> = std::result::Result<T, PortsError>;
