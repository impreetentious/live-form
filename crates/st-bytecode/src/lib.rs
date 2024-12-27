//! Stitch bytecode compiler and verifier.

mod compiler;
mod error;
mod module;
mod op;
mod scope;
mod verify;

pub use compiler::compile;
pub use error::BytecodeError;
pub use module::Module;
pub use op::Op;
pub use scope::resolve_captures;
pub use verify::verify;

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::{compile, verify, Module};

    #[test]
    fn smoke() {
        assert_eq!(env!("CARGO_PKG_NAME"), "st-bytecode");
        assert_eq!(
            compile(&st_syntax::Program::empty()).unwrap_err().code,
            "E0299"
        );
        assert_eq!(verify(&Module::empty()).unwrap_err().code, "E0299");
    }
}
