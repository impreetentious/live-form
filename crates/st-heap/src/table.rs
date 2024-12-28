//! Object table. Slot allocation is not implemented yet.

use crate::error::HeapError;
use crate::object::ObjRef;

/// Generational object table.
#[derive(Debug, Default)]
pub struct ObjectTable {
    _private: (),
}

impl ObjectTable {
    /// Empty table.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Allocate a slot.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until the table is implemented.
    pub fn alloc(&mut self) -> Result<ObjRef, HeapError> {
        Err(HeapError::not_implemented("ObjectTable::alloc"))
    }
}
