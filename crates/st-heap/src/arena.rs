//! Page arena. Allocation is not implemented yet.

use crate::error::HeapError;

/// Normal page size in bytes.
pub const PAGE_BYTES: u32 = 262_144;

/// Page-backed arena.
#[derive(Debug, Default)]
pub struct Arena {
    _private: (),
}

impl Arena {
    /// Empty arena.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Reserve payload bytes.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until the arena is implemented.
    pub fn alloc(&mut self, _size: u32) -> Result<(), HeapError> {
        Err(HeapError::not_implemented("Arena::alloc"))
    }
}
