//! Runtime: parse/compile without executing, then bounded frames.

use crate::budget::Budget;
use crate::error::{BuildError, UpdateError, VmError};
use crate::host::Host;
use crate::report::{FrameReport, UpdateReport};
use crate::update::{SafePoint, UpdateBundle};
use st_heap::Value;

/// Generational pin. An unpinned host `ObjRef` is not a root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PinId(u32);

impl PinId {
    /// Wrap a raw identifier.
    #[must_use]
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/// Stitch runtime. Neither `Send` nor `Sync`.
pub struct Runtime {
    _host: Box<dyn Host>,
}

impl Runtime {
    /// Parse and compile `source` without running guest code or reading time.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until construction is implemented.
    pub fn new(_source: &str, _seed: u64, host: Box<dyn Host>) -> Result<Self, BuildError> {
        let _ = host;
        Err(BuildError::not_implemented("Runtime::new"))
    }

    /// Run one frame.
    #[must_use]
    pub fn frame(&mut self, _budget: Budget) -> FrameReport {
        FrameReport::empty()
    }

    /// Prepare an update without allocating guest objects.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until updates are implemented.
    pub fn prepare_update(&self, _source: &str) -> Result<UpdateBundle, UpdateError> {
        Err(UpdateError::not_implemented("Runtime::prepare_update"))
    }

    /// Apply a previously prepared bundle.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until updates are implemented.
    pub fn apply_update(&mut self, _bundle: UpdateBundle) -> Result<UpdateReport, UpdateError> {
        Err(UpdateError::not_implemented("Runtime::apply_update"))
    }

    /// Heap occupancy map, one byte per 256 KiB unit.
    #[must_use]
    pub fn heap_map(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Queue a coordinated update.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until queued updates are implemented.
    pub fn queue_update(
        &mut self,
        _bundle: UpdateBundle,
        _targets: Vec<SafePoint>,
        _max_frames: u32,
    ) -> Result<(), UpdateError> {
        Err(UpdateError::not_implemented("Runtime::queue_update"))
    }

    /// Take a queued update outcome.
    pub fn take_update_result(&mut self) -> Option<Result<UpdateReport, UpdateError>> {
        None
    }

    /// Pin a value as a strong host root.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until pins are implemented.
    pub fn pin(&mut self, _value: Value) -> Result<PinId, VmError> {
        Err(VmError::not_implemented("Runtime::pin"))
    }

    /// Drop a pin.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until pins are implemented.
    pub fn unpin(&mut self, _pin: PinId) -> Result<(), VmError> {
        Err(VmError::not_implemented("Runtime::unpin"))
    }
}
