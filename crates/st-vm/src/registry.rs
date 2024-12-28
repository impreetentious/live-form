//! Code and global registry. Linking is not implemented yet.

use crate::error::VmError;

/// Install a compiled module.
///
/// # Errors
///
/// Returns `E0299` until the registry is implemented.
pub fn install() -> Result<(), VmError> {
    Err(VmError::not_implemented("registry::install"))
}
