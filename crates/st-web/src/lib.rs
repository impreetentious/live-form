//! Browser adapter for the Liveform runtime.

use wasm_bindgen::prelude::*;

/// JavaScript-facing runtime shell.
#[wasm_bindgen]
pub struct WebRuntime {
    _private: (),
}

#[wasm_bindgen]
impl WebRuntime {
    /// Construct from source, u64 seed, and a `now_us` callback.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until the adapter is implemented.
    #[wasm_bindgen(constructor)]
    pub fn new(_source: &str, _seed: u64, _now: js_sys::Function) -> Result<WebRuntime, JsValue> {
        Err(JsValue::from_str("E0299: WebRuntime is not implemented"))
    }

    /// Run one frame with a microsecond budget.
    ///
    /// # Errors
    ///
    /// Returns `E0299` until the adapter is implemented.
    pub fn frame(&mut self, _budget_us: u32) -> Result<JsValue, JsValue> {
        Err(JsValue::from_str("E0299: frame is not implemented"))
    }

    /// Release wasm state.
    pub fn dispose(&mut self) {}
}

/// Schema of an empty native frame report. Used to keep the VM crate linked.
#[must_use]
pub fn empty_frame_schema() -> u8 {
    let _ = st_heap::PAGE_BYTES;
    st_vm::FrameReport::empty().schema
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-web");
    }
}
