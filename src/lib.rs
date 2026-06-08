#[allow(dead_code)]
pub mod tarnish_compiler;
pub mod compile_error;
pub mod file_builder;

pub use tarnish_compiler::TarnishCompiler;
pub use compile_error::CompileError;
pub use compile_error::FileCompileError;