use crate::FileCompileError;

pub struct FileBuilder {
    filename: String
}

impl FileBuilder {
    pub fn compile(&mut self) -> Result<(), FileCompileError>{
        Ok(())
    }

    pub fn file_name(&self) -> String{
        self.filename.clone()
    }

    pub fn new(filename: String) -> FileBuilder {
        FileBuilder{
            filename
        }
    }
}