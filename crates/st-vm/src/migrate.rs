//! Object recipes. Migration is not implemented yet.

use crate::error::VmError;

/// Run one migration step.
///
/// # Errors
///
/// Returns `E0299` until migration is implemented.
pub fn step() -> Result<(), VmError> {
    Err(VmError::not_implemented("migrate::step"))
}
