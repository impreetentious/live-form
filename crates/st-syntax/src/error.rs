//! Stitch source errors. Codes match the published language/runtime tables.

use thiserror::Error;

/// A diagnostic with a frozen code from the language tables.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{code}: {message}")]
pub struct SyntaxError {
    /// Stable code such as `E0001`.
    pub code: &'static str,
    /// Diagnostic text. Tests assert codes, not this prose.
    pub message: String,
}

impl SyntaxError {
    /// Named future boundary. Never reports success.
    #[must_use]
    pub fn not_implemented(what: &str) -> Self {
        Self {
            code: "E0299",
            message: format!("{what} is not implemented"),
        }
    }

    /// Unexpected token.
    #[must_use]
    pub fn unexpected_token(message: impl Into<String>) -> Self {
        Self {
            code: "E0001",
            message: message.into(),
        }
    }
}

/// Frozen syntax and compile codes declared for this crate's surface.
pub const SYNTAX_CODES: &[&str] = &[
    "E0001", "E0002", "E0003", "E0004", "E0005", "E0006", "E0007", "E0008", "E0100", "E0101",
    "E0102", "E0103", "E0104", "E0105", "E0106", "E0107", "E0108", "E0109", "E0110", "E0111",
    "E0112",
];
