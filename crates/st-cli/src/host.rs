//! Command-line host. Time and output live here, not in library crates.

use st_vm::{DrawCmd, Host, HostError};

/// Terminal/file host used by `run` and `soak`.
pub struct CliHost;

impl Host for CliHost {
    fn now_us(&self) -> u64 {
        0
    }

    fn print(&mut self, _chunk: &str, _end_line: bool) -> Result<(), HostError> {
        Err(HostError::not_implemented("CliHost::print"))
    }

    fn draw(
        &mut self,
        _batch_id: u64,
        _commands: &[DrawCmd],
        _complete: bool,
    ) -> Result<(), HostError> {
        Err(HostError::not_implemented("CliHost::draw"))
    }

    fn canvas_size(&self) -> (i64, i64) {
        (800, 600)
    }
}
