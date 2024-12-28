//! Heap errors.

use thiserror::Error;

/// Allocation or handle failure.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{code}: {message}")]
pub struct HeapError {
    /// Stable code such as `E0211`.
    pub code: &'static str,
    /// Diagnostic text.
    pub message: String,
}

impl HeapError {
    /// Named future boundary.
    #[must_use]
    pub fn not_implemented(what: &str) -> Self {
        Self {
            code: "E0299",
            message: format!("{what} is not implemented"),
        }
    }
}
