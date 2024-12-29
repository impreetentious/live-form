//! Host-side JSON adapters. Serialization stays out of library crates.

use serde::Serialize;

#[derive(Serialize)]
struct ReportEnvelope {
    schema: u8,
}

/// Write a report file. Not implemented yet.
pub fn write_report(_path: &std::path::Path, _body: &str) -> Result<(), String> {
    let _ = serde_json::to_string(&ReportEnvelope { schema: 1 });
    Err("E0299: report write is not implemented".to_owned())
}
