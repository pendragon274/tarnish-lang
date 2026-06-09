#[allow(dead_code)]
pub mod tarnish_compiler;
pub mod compile_error;
pub mod file_builder;
pub mod compile_processor;
pub mod token;

pub use tarnish_compiler::TarnishCompiler;
pub use compile_error::CompileError;
pub use compile_error::FileCompileError;
pub use compile_error::CompileProcessorError;
pub use compile_processor::CompileProcessor;
pub use token::Token;