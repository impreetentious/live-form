//! Guest values. This Rust enum is an API value, never its arena encoding.

use crate::object::ObjRef;

/// Runtime value seen by hosts and the VM.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Guest nil.
    Nil,
    /// Guest boolean.
    Bool(bool),
    /// Guest integer.
    Int(i64),
    /// Guest float. Nonfinite values are traps, not inhabitants.
    Float(f64),
    /// Heap handle.
    Obj(ObjRef),
}
