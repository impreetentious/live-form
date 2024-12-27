//! Diagnostics attached to source spans.

use crate::error::SyntaxError;
use crate::span::Span;

/// One compiler diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    span: Option<Span>,
    error: SyntaxError,
}

impl Diagnostic {
    /// Wrap an error without a span.
    #[must_use]
    pub fn from_error(error: SyntaxError) -> Self {
        Self { span: None, error }
    }

    /// Frozen code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        self.error.code
    }
}
