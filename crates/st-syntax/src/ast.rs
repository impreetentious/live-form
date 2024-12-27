//! Abstract syntax tree. Construction is owned by the parser.

/// Placeholder program node. Fields stay private until the parser lands.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Program {
    _private: (),
}

impl Program {
    /// Empty program used by tests that only need the type.
    #[must_use]
    pub fn empty() -> Self {
        Self { _private: () }
    }
}
