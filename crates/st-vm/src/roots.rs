//! External roots. Registration is not implemented yet.

use crate::error::VmError;
use st_heap::Value;

/// Root identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootId(u32);

/// Register a host root.
///
/// # Errors
///
/// Returns `E0299` until roots are implemented.
pub fn register(_value: Value) -> Result<RootId, VmError> {
    Err(VmError::not_implemented("roots::register"))
}
