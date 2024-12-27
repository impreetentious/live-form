//! Recursive-descent parser. Parsing is not implemented yet.

use crate::ast::Program;
use crate::error::SyntaxError;

/// Parse `source` into a program.
///
/// # Errors
///
/// Returns [`SyntaxError`] with `E0299` until the parser is implemented.
pub fn parse(_source: &str) -> Result<Program, SyntaxError> {
    Err(SyntaxError::not_implemented("parse"))
}
