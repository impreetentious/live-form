//! Lexical scope analysis. Capture resolution is not implemented yet.

use crate::error::BytecodeError;
use st_syntax::Program;

/// Analyze captures for a parsed program.
///
/// # Errors
///
/// Returns `E0299` until scope analysis is implemented.
pub fn resolve_captures(_program: &Program) -> Result<(), BytecodeError> {
    Err(BytecodeError::not_implemented("resolve_captures"))
}
