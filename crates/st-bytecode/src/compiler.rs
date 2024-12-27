//! Bytecode compiler. Emission is not implemented yet.

use crate::error::BytecodeError;
use crate::module::Module;
use st_syntax::Program;

/// Compile a parsed program into a module.
///
/// # Errors
///
/// Returns `E0299` until the compiler is implemented.
pub fn compile(_program: &Program) -> Result<Module, BytecodeError> {
    Err(BytecodeError::not_implemented("compile"))
}
