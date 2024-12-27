//! Closed instruction set. Encoding lands with the compiler.

/// Instruction index operand sizes match the language tables.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    /// Push nil.
    Nil,
    /// Explicit yield. Operand is the label sentinel.
    Yield(u32),
}
