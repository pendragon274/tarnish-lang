#[derive(Debug, Clone)]
pub struct Token(String);

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType{
    Empty,
    Letter,
    Number,
    Operator,
    Whitespace,
    Other
}

impl Token{
    // ***** Public Functions *****
    pub fn tokenize(s: &str) -> Vec<Token>{
        let split = s.split("").map(|s| Token::from(s.to_string())).collect::<Vec<Token>>();
        Self::compress(split)
    }

    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }

    pub fn token_type(&self) -> TokenType{
        if self.is_empty(){
            TokenType::Empty
        }else{
            let my_char: char = self.0.chars().nth(0).unwrap();
            if my_char >= '0' && my_char <= '9'{
                TokenType::Number
            }else if (my_char >= 'a' && my_char <= 'z') || (my_char >= 'A' && my_char <= 'Z'){
                TokenType::Letter
            }else if my_char == '\\' || my_char == '"' || my_char == '\'' || my_char == '(' || my_char == ')' || my_char == '~' ||
                my_char == '{' || my_char == '}' || my_char == '[' || my_char == ']' || my_char == '!' || my_char == '@' || my_char == '#' || my_char == '$' || my_char == '%' ||
                my_char == '<' || my_char == '>' || my_char == ',' || my_char == '.' || my_char == '?' || my_char == '/' || my_char == '^' || my_char == '&' || my_char == '*'{
                TokenType::Operator
            }else if my_char == ' ' || my_char == '\t' || my_char == '\n' || my_char == '\r'{
                TokenType::Whitespace
            }else{
                TokenType::Other
            }
        }
    }

    // ***** Private Functions *****
    fn try_join(&mut self, tok: Token) -> Result<(), ()>{
        if self.token_type() == tok.token_type(){
            self.0.extend(tok.0.chars());
            Ok(())
        }else {
            Err(())
        }
    }

    fn remove_empty(vec: Vec<Token>) -> Vec<Token>{
        let mut ret: Vec<Token> = Vec::with_capacity(vec.len());
        for tok in vec{
            if !tok.is_empty(){
                ret.push(tok);
            }
        }

        ret
    }

    fn compress(vec: Vec<Token>) -> Vec<Token>{
        let no_empty = Self::remove_empty(vec);
        if no_empty.is_empty(){
            return Vec::new();
        }

        let mut ret: Vec<Token> = Vec::with_capacity(no_empty.len());
        let mut cur_tok: (usize, Token) = (0, no_empty[0].clone());
        let mut comp_idx: usize = 1;
        loop{
            if comp_idx >= no_empty.len(){
                let (_, tok) = cur_tok;
                ret.push(tok);
                break;
            }else{
                match cur_tok.1.try_join(no_empty[comp_idx].clone()){
                    Ok(()) =>{
                        comp_idx += 1;
                    }, Err(()) =>{
                        let (idx, tok) = cur_tok;
                        ret.push(tok);
                        cur_tok = (comp_idx, no_empty[comp_idx].clone());
                        comp_idx += 1;
                    }
                }
            }
        }

        ret
    }

    // ***** Struct Init *****
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token(s)
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Token(String::from(s))
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        self.0.eq(&other.0)
    }
}