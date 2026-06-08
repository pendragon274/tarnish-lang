use crate::token::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SymbolTable {
    symbols: Vec<Token>,
    resolution: Vec<Token>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable{
            symbols: Vec::new(),
            resolution: Vec::new()
        }
    }
}