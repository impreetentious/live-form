//! Incremental collector. Tracing is not implemented yet.

use crate::error::HeapError;

/// Collector states from the heap tables.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GcState {
    /// No cycle in progress.
    Idle,
}

/// Incremental tracing and sweep.
#[derive(Debug, Default)]
pub struct Collector {
    _private: (),
}

impl Collector {
    /// Idle collector.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Run one bounded step.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until GC is implemented.
    pub fn step(&mut self) -> Result<(), HeapError> {
        Err(HeapError::not_implemented("Collector::step"))
    }
}
