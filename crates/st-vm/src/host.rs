//! Host callbacks. Library crates never print or read time themselves.

use crate::error::HostError;

/// Draw command. Coordinates stay finite in later owners.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCmd {
    /// Clear the canvas.
    Clear {
        /// RGB 0..=0xffffff.
        rgb: u32,
    },
    /// Filled rectangle.
    Rect {
        /// Origin x.
        x: f64,
        /// Origin y.
        y: f64,
        /// Width.
        w: f64,
        /// Height.
        h: f64,
        /// RGB 0..=0xffffff.
        rgb: u32,
    },
    /// Text run.
    Text {
        /// Origin x.
        x: f64,
        /// Origin y.
        y: f64,
        /// Payload. Capped at 4000 UTF-8 bytes by the interpreter.
        text: String,
    },
}

/// Host supplied to a runtime.
pub trait Host {
    /// Monotonic microseconds. Must not be called under a Units budget.
    fn now_us(&self) -> u64;

    /// Print a UTF-8 chunk.
    ///
    /// # Errors
    ///
    /// Returns `E0212` when the host cannot accept output.
    fn print(&mut self, chunk: &str, end_line: bool) -> Result<(), HostError>;

    /// Submit a draw batch chunk.
    ///
    /// # Errors
    ///
    /// Returns `E0212` when the host cannot accept drawing.
    fn draw(
        &mut self,
        batch_id: u64,
        commands: &[DrawCmd],
        complete: bool,
    ) -> Result<(), HostError>;

    /// Guest canvas size. Native default 800×600.
    fn canvas_size(&self) -> (i64, i64);
}
