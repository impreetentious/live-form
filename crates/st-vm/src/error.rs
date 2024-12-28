//! VM and host errors.

use thiserror::Error;

/// Build, run, host, or update failure.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{code}: {message}")]
pub struct VmError {
    /// Stable code such as `E0299`.
    pub code: &'static str,
    /// Diagnostic text.
    pub message: String,
}

impl VmError {
    /// Named future boundary.
    #[must_use]
    pub fn not_implemented(what: &str) -> Self {
        Self {
            code: "E0299",
            message: format!("{what} is not implemented"),
        }
    }
}

/// Error from `Runtime::new`.
pub type BuildError = VmError;
/// Error from host callbacks.
pub type HostError = VmError;
/// Error from prepare/apply/queue update.
pub type UpdateError = VmError;
