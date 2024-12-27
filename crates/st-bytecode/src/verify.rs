//! Bytecode verifier. Checking is not implemented yet.

use crate::error::BytecodeError;
use crate::module::Module;

/// Verify a compiled module.
///
/// # Errors
///
/// Returns `E0299` until the verifier is implemented.
pub fn verify(_module: &Module) -> Result<(), BytecodeError> {
    Err(BytecodeError::not_implemented("verify"))
}
