//! Heap values, handles, and collectors.

mod arena;
mod error;
mod object;
mod shape;
mod stats;
mod table;
mod value;

pub use arena::{Arena, PAGE_BYTES};
pub use error::HeapError;
pub use object::ObjRef;
pub use shape::{Shape, ShapeId};
pub use stats::HeapStats;
pub use table::ObjectTable;
pub use value::Value;

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::{ObjRef, Value, PAGE_BYTES};

    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-heap");
        assert_eq!(PAGE_BYTES, 262_144);
        let handle = ObjRef::new(1, 1).expect("valid handle");
        assert_eq!(handle.slot(), 1);
        assert!(ObjRef::new(1, 0).is_none());
        assert!(matches!(Value::Nil, Value::Nil));
    }
}
