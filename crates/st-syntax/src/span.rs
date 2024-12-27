//! Byte offsets and one-based display positions.

/// Inclusive start, exclusive end, in UTF-8 bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    /// Build a span. `end` must be ≥ `start`.
    #[must_use]
    pub fn new(start: u32, end: u32) -> Option<Self> {
        if end < start {
            return None;
        }
        Some(Self { start, end })
    }

    /// Byte offset of the first included byte.
    #[must_use]
    pub fn start(self) -> u32 {
        self.start
    }

    /// Byte offset one past the last included byte.
    #[must_use]
    pub fn end(self) -> u32 {
        self.end
    }
}
