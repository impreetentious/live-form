//! `stitch test` corpus runner.

use std::path::PathBuf;
use std::process::ExitCode;

/// Run fixtures under `dir`.
pub fn test(_dir: PathBuf) -> ExitCode {
    eprintln!("E0299: test is not implemented");
    ExitCode::from(5)
}
