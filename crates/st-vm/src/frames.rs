//! Suspended-frame descriptors. Transfer is not implemented yet.

use crate::error::VmError;

/// Migrate one suspended frame.
///
/// # Errors
///
/// Returns `E0299` until frame migration is implemented.
pub fn migrate_frame() -> Result<(), VmError> {
    Err(VmError::not_implemented("frames::migrate"))
}
