use crate::CompileProcessorError;
use crate::Token;

pub struct CompileProcessor{
    preprocess_tokens: Vec<Token>,
    process_token_index: usize,
    preprocess_line_number_mapping: Vec<(usize, usize)>
}

impl CompileProcessor{
    // ***** Public Functions *****
    pub fn input(&mut self, contents: Vec<String>) -> Result<(), CompileProcessorError>{
        for (index, line) in contents.iter().enumerate(){
            let start_mapping = self.preprocess_tokens.len();
            let tokenize = Token::tokenize(line);
            println!("Tokenizing line:\n{:?}\ninto\n{:?}", line, tokenize);
            self.preprocess_tokens.extend(tokenize);
            self.preprocess_line_number_mapping.push((index, start_mapping));
        }
        
        Err(CompileProcessorError::new(None, "Not implemented yet!", 0))
    }

    pub fn output(&mut self) -> Result<Vec<(String, usize)>, CompileProcessorError>{
        Err(CompileProcessorError::new(None, "Not implemented yet!", 0))
    }

    // ***** Private Functions *****

    // ***** Struct Init *****
    pub fn new() -> CompileProcessor{
        CompileProcessor{
            preprocess_tokens: Vec::new(),
            process_token_index: 0,
            preprocess_line_number_mapping: Vec::new()
        }
    }
}