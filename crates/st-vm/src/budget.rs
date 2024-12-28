//! Work budgets. Time and work counters are `u64`.

/// Frame work bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Budget {
    /// Interpreter work units.
    Units(u64),
    /// Microseconds of accounted runtime.
    Micros(u64),
}
