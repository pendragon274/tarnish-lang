#[derive(Debug)]
pub struct Token(String);

impl Token{
    pub fn separate(s: &str) -> Vec<Token>{
        let whitespace_separated: Vec<String> = s.split_whitespace().map(|s| String::from(s)).collect();
        let mut ret: Vec<Token> = Vec::new();
        for token in whitespace_separated.iter() {
            for item in token.split(&['`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{', '}', '|', '\\', '\"', '\'', ':', ';', '<', '>', ',', '.', '/', '?']){
                ret.push(Token::from(item));
            }
        }

        ret
    }
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