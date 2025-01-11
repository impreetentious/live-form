//! Integration: CLI help lists the five commands and run/soak flags.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

fn output(args: &[&str]) -> String {
    let exe = env!("CARGO_BIN_EXE_stitch");
    let out = std::process::Command::new(exe)
        .args(args)
        .output()
        .expect("spawn stitch");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn help_lists_commands_and_flags() {
    let help = output(&["--help"]);
    for command in ["check", "dump", "test", "run", "soak"] {
        assert!(help.contains(command), "missing {command}");
    }

    let run = output(&["run", "--help"]);
    for flag in [
        "--seed",
        "--deterministic",
        "--frames",
        "--budget-us",
        "--budget-units",
        "--max-pages",
        "--update",
        "--report",
        "--draw-stats",
        "--verbose",
    ] {
        assert!(run.contains(flag), "run missing {flag}");
    }

    let soak = output(&["soak", "--help"]);
    for flag in ["--seconds", "--assert"] {
        assert!(soak.contains(flag), "soak missing {flag}");
    }

    let dump = output(&["dump", "--help"]);
    assert!(dump.contains("--ast"));
    assert!(dump.contains("--bytecode"));
}
