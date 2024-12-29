//! Stitch virtual machine.

mod budget;
#[allow(dead_code)]
mod builtins;
#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod fiber;
#[allow(dead_code)]
mod frames;
mod host;
#[allow(dead_code)]
mod interp;
#[allow(dead_code)]
mod migrate;
mod numeric;
#[allow(dead_code)]
mod registry;
mod report;
#[allow(dead_code)]
mod roots;
mod runtime;
#[allow(dead_code)]
mod sched;
#[allow(dead_code)]
mod services;
#[allow(dead_code)]
mod update;

pub use budget::Budget;
pub use error::{BuildError, HostError, UpdateError, VmError};
pub use fiber::FiberId;
pub use host::{DrawCmd, Host};
pub use numeric::{cos, floor, sin, sqrt};
pub use report::{FrameReport, UpdateReport};
pub use runtime::{PinId, Runtime};
pub use update::{SafePoint, UpdateBundle};

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
