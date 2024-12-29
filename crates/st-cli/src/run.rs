//! `stitch run`.

use crate::host::CliHost;
use crate::report::write_report;
use std::path::PathBuf;
use std::process::ExitCode;

/// Run flags shared with soak except soak-only switches.
#[allow(dead_code)]
pub struct RunOptions {
    /// Program path.
    pub file: PathBuf,
    /// RNG seed. Default 1.
    pub seed: u64,
    /// Deterministic Units mode.
    pub deterministic: bool,
    /// Exact frame count. `None` runs until guest fibers finish.
    pub frames: Option<u64>,
    /// Microsecond budget. Default 8000.
    pub budget_us: u64,
    /// Unit budget when deterministic. Default 1_000_000.
    pub budget_units: u64,
    /// Arena page cap. Default 4096.
    pub max_pages: u32,
    /// Repeatable `FRAME:FILE` updates.
    pub updates: Vec<String>,
    /// Optional report path.
    pub report: Option<PathBuf>,
    /// Include draw hashes in the report.
    pub draw_stats: bool,
}

/// Execute a program.
pub fn run(opts: RunOptions) -> ExitCode {
    let _host = CliHost;
    let _ = st_heap::PAGE_BYTES;
    if let Some(path) = opts.report.as_deref() {
        let _ = write_report(path, "{}");
    }
    eprintln!("E0299: run is not implemented");
    ExitCode::from(1)
}
