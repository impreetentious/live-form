//! Incremental compaction. Copying is not implemented yet.

use crate::error::HeapError;

/// Compaction cursor.
#[derive(Debug, Default)]
pub struct Compactor {
    _private: (),
}

impl Compactor {
    /// Idle compactor.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Copy one bounded chunk.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until compaction is implemented.
    pub fn step(&mut self) -> Result<(), HeapError> {
        Err(HeapError::not_implemented("Compactor::step"))
    }
}
