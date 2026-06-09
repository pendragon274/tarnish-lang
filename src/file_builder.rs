use std::{fs, usize};
use std::process::Command;
use crate::{CompileProcessorError, FileCompileError};
use crate::CompileProcessor;

pub struct FileBuilder {
    filename: String,
    original_contents: Option<Vec<String>>,
    processed_contents: Option<Vec<(String, usize)>>,
    processor: CompileProcessor
}

impl FileBuilder {
    // ***** Public Functions *****
    pub fn compile(&mut self) -> Result<(), FileCompileError>{
        let name = self.filename.clone();
        println!("\nCompiling {}...", name);
        let contents = match &self.original_contents {
            Some(text) => text.clone(),
            None => return Err(FileCompileError::new(None, name, "File not found.", 1))
        };

        self.process(contents)?;

        Ok(())
    }

    pub fn write_ll(&mut self) -> Result<(), FileCompileError>{
        match &self.processed_contents{
            Some(contents) => {
                let name = self.ll_name();
                println!("\nWriting intermediate file {}...", name);
                let write_result = fs::write(Self::working_directory() + &name, contents.iter().map(|(line, _index)| line.clone()).collect::<Vec<String>>().join("\n"));
                match write_result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(FileCompileError::new(None, name, "Unable to write to file.", 2))
                }
            }, None => Err(FileCompileError::new(None, self.filename.clone(), "File not compiled.", 3))
        }
    }

    pub fn remove_ll(&mut self) -> Result<(), FileCompileError>{
        if let Ok(exists) = fs::exists(Self::working_directory() + self.ll_name().as_str()){
            if !exists{
                return Err(FileCompileError::new(None, self.filename.clone(), "Intermediate assembly file does not exist.", 4));
            }
        }else{
            return Err(FileCompileError::new(None, self.filename.clone(), "Could not interact with intermediate assembly file.", 5));
        }

        println!("Removing intermediate assembly file {}...", self.ll_name());
        let rm_output = Command::new("rm").args([self.ll_name().as_str()]).output();
        match rm_output{
            Ok(output) => {
                if output.stderr.len() > 0{
                    Err(self.extract_error(String::from_utf8(output.stderr).unwrap()))
                }else{
                    Ok(())
                }
            }, Err(_) => {
                Err(FileCompileError::new(None, self.filename.clone(), "Could not execute the command to remove the intermediate assembly file.", 7))
            }
        }
    }

    pub fn ll_compile(&mut self) -> Result<(), FileCompileError>{
        if let Ok(exists) = fs::exists(Self::working_directory() + self.ll_name().as_str()){
            if !exists {
                return Err(FileCompileError::new(None, self.filename.clone(), "Intermediate assembly file does not exist.", 4));
            }
        }else{
            return Err(FileCompileError::new(None, self.filename.clone(), "Could not interact with intermediate assembly file.", 5));
        }

        println!("Compiling intermediate assembly file {}...", self.ll_name());
        let llc_output = Command::new("llc").args(["-filetype=obj", self.ll_name().as_str(), "-o", self.o_name().as_str()]).output();
        match llc_output{
            Ok(output) => {
                if output.stderr.len() > 0{
                    Err(self.extract_error(String::from_utf8(output.stderr).unwrap()))
                    //Err(FileCompileError::new(None, self.filename.clone(), String::from_utf8(output.stderr).unwrap() + "\nThere should be line information and such somewhere in here.", 6))
                }else{
                    Ok(())
                }
            }, Err(_) => {
                Err(FileCompileError::new(None, self.filename.clone(), "Could not execute the command to compile the intermediate assembly file.", 7))
            }
        }
    }

    pub fn file_name(&self) -> String{
        self.filename.clone()
    }

    // ***** Private Functions *****
    fn convert_result<T>(&self, result: Result<T, CompileProcessorError>) -> Result<T, FileCompileError>{
        match result{
            Ok(value) => Ok(value),
            Err(e) => Err(FileCompileError::new(e.line(), self.filename.clone(), e.message().as_str(), e.code()))
        }
    }

    fn process(&mut self, contents: Vec<String>) -> Result<(), FileCompileError>{
        let input_result = self.processor.input(contents);
        self.convert_result(input_result)?;
        let output_result = self.processor.output();
        self.processed_contents = Some(self.convert_result(output_result)?);

        Ok(())
    }

    fn extract_error(&self, stderr_output: String) -> FileCompileError{
        let line_info_location = stderr_output.find(&(self.ll_name() + ":"));
        match line_info_location{
            None => FileCompileError::new(None, self.filename.clone(), ("Unknown intermediate assembly compile error: ".to_string() + &stderr_output).as_str(), 6),
            Some(index) => {
                let slice = &stderr_output[(index + self.ll_name().len() + 1)..];
                //println!("The original slice: {}", slice.to_string());
                let next_find = slice.find(":");
                match next_find{
                    None => FileCompileError::new(None, self.filename.clone(), ("Unknown line number interpretation from intermediate asembly compile error: ".to_string() + &stderr_output).as_str(), 8),
                    Some(end_index) => {
                        let line_number = usize::from(slice[..end_index].parse::<usize>().unwrap());
                        let end = slice.rfind(":").unwrap();
                        FileCompileError::new(Some(line_number), self.filename.clone(), slice[end + 1..].to_string().as_str(), 9)
                    }
                }
            }
        }
    }

    fn ll_name(&self) -> String{
        let found_delimiter = self.filename.find(".");
        match found_delimiter {
            None => self.filename.clone() + ".ll",
            Some(index) => String::from(&self.filename[0..index]) + ".ll"
        }
    }

    fn o_name(&self) -> String{
        let found_delimiter = self.filename.find(".");
        match found_delimiter{
            None => self.filename.clone() + ".o",
            Some(index) => String::from(&self.filename[0..index]) + ".o"
        }
    }

    fn working_directory() -> String{
        std::env::current_dir().unwrap().to_str().unwrap().to_string() + "/"
    }

    // ***** Struct Init *****
    pub fn new(new_filename: String) -> FileBuilder {
        let contents = fs::read_to_string(Self::working_directory() + &new_filename);

        match contents {
            Ok(contents) => {
                FileBuilder {
                    filename: new_filename.clone(),
                    original_contents: Some(contents.split('\n').map(|s| s.to_string()).collect()),
                    processed_contents: None,
                    processor: CompileProcessor::new()
                }
            }, Err(_) => {
                FileBuilder{
                    filename: new_filename,
                    original_contents: None,
                    processed_contents: None,
                    processor: CompileProcessor::new()
                }
            }
        }
    }
}