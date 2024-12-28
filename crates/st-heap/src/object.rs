//! Object-table handles.

use core::num::NonZeroU64;

/// Generational object-table handle. High 32 bits generation, low 32 slot; slot 0 invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ObjRef(NonZeroU64);

impl ObjRef {
    /// Pack generation and slot. Slot `0` is invalid.
    #[must_use]
    pub fn new(generation: u32, slot: u32) -> Option<Self> {
        if slot == 0 {
            return None;
        }
        let packed = (u64::from(generation) << 32) | u64::from(slot);
        NonZeroU64::new(packed).map(Self)
    }

    /// Generation bits.
    #[must_use]
    pub fn generation(self) -> u32 {
        (self.0.get() >> 32) as u32
    }

    /// Slot bits.
    #[must_use]
    pub fn slot(self) -> u32 {
        self.0.get() as u32
    }
}
