//! Interpreter loop. Execution is not implemented yet.

use crate::error::VmError;

/// Step the interpreter.
///
/// # Errors
///
/// Returns `E0299` until the interpreter is implemented.
pub fn step() -> Result<(), VmError> {
    Err(VmError::not_implemented("interp::step"))
}
