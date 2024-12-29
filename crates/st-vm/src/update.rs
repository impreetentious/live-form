//! Transactional code/global updates.

use crate::error::UpdateError;

/// Prepared update. Owner token and compiled diff land with the updater.
#[derive(Debug)]
pub struct UpdateBundle {
    _private: (),
}

/// Function and label selecting live fibers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafePoint {
    /// Function name.
    pub function: String,
    /// Yield label.
    pub label: String,
}

/// Prepare an update from source.
///
/// # Errors
///
/// Returns `E0299` until updates are implemented.
pub fn prepare(_source: &str) -> Result<UpdateBundle, UpdateError> {
    Err(UpdateError::not_implemented("update::prepare"))
}
