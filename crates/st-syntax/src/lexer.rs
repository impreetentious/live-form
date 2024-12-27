//! Lexer. Tokenization is not implemented yet.

use crate::error::SyntaxError;

/// A source token. The token kind table lands with the lexer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    _private: (),
}

/// Lex `source` into tokens.
///
/// # Errors
///
/// Returns [`SyntaxError`] with `E0299` until the lexer is implemented.
pub fn lex(_source: &str) -> Result<Vec<Token>, SyntaxError> {
    Err(SyntaxError::not_implemented("lex"))
}
