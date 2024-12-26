//! `stitch` command line.

mod host;
mod report;
mod run;
mod soak;
mod test;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{ArgAction, Args, Parser, Subcommand};

use run::RunOptions;

/// Stitch language and runtime.
#[derive(Parser, Debug)]
#[command(name = "stitch", version = env!("CARGO_PKG_VERSION"), about = "Stitch language and runtime")]
struct Cli {
    /// Increase log verbosity. Repeat for info, debug, trace.
    #[arg(short, long, action = ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse a source file without compiling.
    Check {
        /// Source path.
        file: PathBuf,
    },
    /// Print AST or bytecode.
    Dump {
        /// Source path.
        file: PathBuf,
        /// Print the AST.
        #[arg(long, group = "dump_kind")]
        ast: bool,
        /// Print bytecode.
        #[arg(long, group = "dump_kind")]
        bytecode: bool,
    },
    /// Run a corpus directory.
    Test {
        /// Fixture directory.
        dir: PathBuf,
    },
    /// Run a program.
    Run(RunCli),
    /// Run a timed or framed soak.
    Soak(SoakCli),
}

#[derive(Args, Debug)]
struct RunCli {
    /// Source path.
    file: PathBuf,
    /// RNG seed.
    #[arg(long, default_value_t = 1)]
    seed: u64,
    /// Deterministic Units mode. Implies `--budget-units`.
    #[arg(long)]
    deterministic: bool,
    /// Exact number of frame calls.
    #[arg(long)]
    frames: Option<u64>,
    /// Microsecond budget. Default 8000. Incompatible with deterministic mode.
    #[arg(long, default_value_t = 8000)]
    budget_us: u64,
    /// Unit budget when deterministic. Default 1,000,000.
    #[arg(long, default_value_t = 1_000_000)]
    budget_units: u64,
    /// Maximum arena pages. Default 4096.
    #[arg(long, default_value_t = 4096)]
    max_pages: u32,
    /// Scheduled update as `FRAME:FILE`. Repeatable.
    #[arg(long = "update", value_name = "FRAME:FILE", action = ArgAction::Append)]
    updates: Vec<String>,
    /// Write a JSON report to this path.
    #[arg(long)]
    report: Option<PathBuf>,
    /// Include draw hashes.
    #[arg(long)]
    draw_stats: bool,
}

#[derive(Args, Debug)]
struct SoakCli {
    #[command(flatten)]
    run: RunCli,
    /// Wall-clock seconds. Mutually exclusive with `--frames`.
    #[arg(long)]
    seconds: Option<u64>,
    /// Exit 5 when soak bounds fail.
    #[arg(long)]
    assert: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    init_log(cli.verbose);
    match cli.command {
        Command::Check { file: _ } => {
            let _ = st_syntax::SYNTAX_CODES.len();
            eprintln!("E0299: check is not implemented");
            ExitCode::from(3)
        }
        Command::Dump { .. } => {
            let _ = st_bytecode::Module::empty();
            eprintln!("E0299: dump is not implemented");
            ExitCode::from(3)
        }
        Command::Test { dir } => test::test(dir),
        Command::Run(args) => run::run(into_run(args)),
        Command::Soak(args) => soak::soak(soak::SoakOptions {
            run: into_run(args.run),
            seconds: args.seconds,
            assert: args.assert,
        }),
    }
}

fn into_run(args: RunCli) -> RunOptions {
    RunOptions {
        file: args.file,
        seed: args.seed,
        deterministic: args.deterministic,
        frames: args.frames,
        budget_us: args.budget_us,
        budget_units: args.budget_units,
        max_pages: args.max_pages,
        updates: args.updates,
        report: args.report,
        draw_stats: args.draw_stats,
    }
}

fn init_log(verbose: u8) {
    let level = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level))
        .format_timestamp(None)
        .try_init();
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-cli");
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }
}
