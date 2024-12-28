//! Cooperative scheduler. Fairness lands with fibers.

use crate::error::VmError;

/// Run ready fibers within a budget.
///
/// # Errors
///
/// Returns `E0299` until scheduling is implemented.
pub fn schedule() -> Result<(), VmError> {
    Err(VmError::not_implemented("sched::schedule"))
}
