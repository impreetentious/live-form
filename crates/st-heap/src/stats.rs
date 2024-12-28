//! Heap occupancy counters.

/// Snapshot of bytes and page counts. All fields start at zero.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeapStats {
    /// Allocated payload bytes of unswept objects.
    pub bytes_live: u64,
    /// Occupied normal pages.
    pub normal_pages_occupied: u32,
}

impl HeapStats {
    /// All-zero counters.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }
}
