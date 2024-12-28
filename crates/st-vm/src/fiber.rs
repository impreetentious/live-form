//! Fibers. Spawning is not implemented yet.

use crate::error::VmError;

/// Guest fiber identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FiberId(u32);

impl FiberId {
    /// Wrap a raw identifier.
    #[must_use]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Raw identifier.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

/// Create a fiber.
///
/// # Errors
///
/// Returns `E0299` until fibers are implemented.
pub fn spawn() -> Result<FiberId, VmError> {
    Err(VmError::not_implemented("fiber::spawn"))
}
