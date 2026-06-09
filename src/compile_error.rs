use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct CompileError{
    errors: Vec<FileCompileError>
}

pub struct FileCompileError{
    line: Option<usize>,
    file: String,
    message: String,
    code: u8
}

pub struct CompileProcessorError{
    line: Option<usize>,
    message: String,
    code: u8
}


impl Debug for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nCompilation contained errors:\n")?;

        for error in &self.errors {
            write!(f, "{:?}", error)?;
        }

        write!(f, "\n")
    }
}

impl Debug for FileCompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(f, "File {}, Code {}, Line {}: {}", self.file, self.code, line, self.message),
            None => write!(f, "File {}, Code {}: {}", self.file, self.code, self.message)
        }
    }
}

impl Display for CompileError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nCompilation contained errors:\n")?;

        for error in &self.errors {
            write!(f, "{}", error)?;
        }

        write!(f, "\n")
    }
}

impl Display for FileCompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(f, "File {}, Line {}: {}", self.file, line, self.message),
            None => write!(f, "File {}: {}", self.file, self.message)
        }
    }
}

impl Error for CompileError{}
impl Error for FileCompileError{}

impl CompileError{
    pub fn code(&self) -> u8{
        self.errors.len() as u8
    }

    pub fn new(errors: Vec<FileCompileError>)->CompileError{
        CompileError{ errors }
    }
}

impl FileCompileError {
    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn new(new_line: Option<usize>, new_file: String, new_message: &str, new_code: u8) -> FileCompileError{
        FileCompileError{ 
            line: new_line,
            file: new_file,
            message: String::from(new_message),
            code: new_code 
        }
    }
}

impl CompileProcessorError {
    pub fn line(&self) -> Option<usize>{
        self.line
    }
    
    pub fn code(&self) -> u8{
        self.code
    }
    
    pub fn message(&self) -> String{
        self.message.clone()
    }
    
    pub fn new(new_line: Option<usize>, new_message: &str, new_code: u8) -> CompileProcessorError {
        CompileProcessorError{
            line: new_line,
            message: String::from(new_message),
            code: new_code
        }
    }
}