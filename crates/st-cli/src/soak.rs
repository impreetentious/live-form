//! `stitch soak`.

use std::process::ExitCode;

use crate::run::RunOptions;

/// Soak-only flags.
#[allow(dead_code)]
pub struct SoakOptions {
    /// Shared run flags.
    pub run: RunOptions,
    /// Wall-clock seconds. Mutually exclusive with frames.
    pub seconds: Option<u64>,
    /// Fail on bound violations.
    pub assert: bool,
}

/// Long-running Colony/workload driver.
pub fn soak(_opts: SoakOptions) -> ExitCode {
    eprintln!("E0299: soak is not implemented");
    ExitCode::from(1)
}
