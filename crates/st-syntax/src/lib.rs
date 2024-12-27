//! Stitch lexer, parser, and diagnostics.

mod ast;
mod diag;
mod error;
mod lexer;
mod parser;
mod span;

pub use ast::Program;
pub use diag::Diagnostic;
pub use error::{SyntaxError, SYNTAX_CODES};
pub use lexer::{lex, Token};
pub use parser::parse;
pub use span::Span;

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::{lex, parse, SYNTAX_CODES};

    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-syntax");
        assert!(SYNTAX_CODES.contains(&"E0001"));
        assert_eq!(lex("").unwrap_err().code, "E0299");
        assert_eq!(parse("").unwrap_err().code, "E0299");
    }
}
