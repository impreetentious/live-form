//! Builtin dispatch. Call sites land with the compiler.

use crate::error::VmError;

/// Invoke a builtin by frozen ID.
///
/// # Errors
///
/// Returns `E0299` until builtins are implemented.
pub fn call_builtin(_id: u16) -> Result<(), VmError> {
    Err(VmError::not_implemented("builtins::call"))
}
