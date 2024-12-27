//! Compiled module container. Linking lands with the compiler.

/// A compiled module. Empty until compilation is implemented.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Module {
    _private: (),
}

impl Module {
    /// Placeholder module for type-level tests.
    #[must_use]
    pub fn empty() -> Self {
        Self { _private: () }
    }
}
