//! Bytecode compile and verify errors.

use thiserror::Error;

/// Compile or verify failure.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{code}: {message}")]
pub struct BytecodeError {
    /// Stable code such as `E0112`.
    pub code: &'static str,
    /// Diagnostic text.
    pub message: String,
}

impl BytecodeError {
    /// Named future boundary.
    #[must_use]
    pub fn not_implemented(what: &str) -> Self {
        Self {
            code: "E0299",
            message: format!("{what} is not implemented"),
        }
    }
}
