use crate::file_builder::FileBuilder;
use crate::compile_error::CompileError;
use crate::compile_error::FileCompileError;

pub struct TarnishCompiler{
    file_builds: Vec<FileBuilder>
}

impl TarnishCompiler{
    pub fn compile(&mut self) -> Result<(), CompileError>{
        println!("\nCompiling files: {:?}", self.file_names_list());

        let mut compile_results: Vec<FileCompileError> = Vec::new();
        let mut success = true;
        for file in &mut self.file_builds{
            let result = file.compile();

            if result.is_err(){
                success = false;
                compile_results.push(result.err().unwrap());
            }
        }

        if !success{
            return Err(CompileError::new(compile_results));
        }

        compile_results.clear();
        success = true;
        for file in &mut self.file_builds{
            let result = file.write_ll();

            if result.is_err(){
                success = false;
                compile_results.push(result.err().unwrap());
            }
        }

        if !success{
            return Err(CompileError::new(compile_results));
        }
        
        compile_results.clear();
        success = true;
        for file in &mut self.file_builds{
            let result = file.ll_compile();
            
            if result.is_err(){
                success = false;
                compile_results.push(result.err().unwrap());
            }
        }
        
        if !success{
            return Err(CompileError::new(compile_results));
        }
        
        compile_results.clear();
        success = true;
        for file in &mut self.file_builds{
            let result = file.remove_ll();
            
            if result.is_err(){
                success = false;
                compile_results.push(result.err().unwrap());
            }
        }
        
        if !success{
            return Err(CompileError::new(compile_results));
        }

        Ok(())
    }

    fn main_file_name(&self) -> String{
        self.file_builds[0].file_name()
    }

    fn file_names_list(&self) -> Vec<String>{
        let mut ret: Vec<String> = Vec::new();

        for file in &self.file_builds{
            ret.push(file.file_name());
        }

        ret
    }

    pub fn new(files: Vec<String>) -> TarnishCompiler{
        TarnishCompiler{
            file_builds: files.iter().map(|file| FileBuilder::new(file.clone())).collect()
        }
    }
}