//! Background services (GC, compaction, migration).

use crate::error::VmError;

/// Run one service slice.
///
/// # Errors
///
/// Returns `E0299` until services are implemented.
pub fn pump() -> Result<(), VmError> {
    Err(VmError::not_implemented("services::pump"))
}
