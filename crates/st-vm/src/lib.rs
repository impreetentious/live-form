//! Stitch virtual machine.

mod budget;
#[allow(dead_code)]
mod error;
mod host;
mod numeric;
mod report;

pub use budget::Budget;
pub use error::{BuildError, HostError, UpdateError, VmError};
pub use host::{DrawCmd, Host};
pub use numeric::{cos, floor, sin, sqrt};
pub use report::{FrameReport, UpdateReport};

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::{sqrt, FrameReport, VmError};

    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-vm");
        assert_eq!(sqrt(4.0), 2.0);
        assert_eq!(FrameReport::empty().schema, 1);
        assert_eq!(VmError::not_implemented("x").code, "E0299");
    }
}
