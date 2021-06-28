use std::fmt;
use super::tokenizer::Tokenizer;
use super::token::Token;



pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of Parser

impl<'a> Parser<'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    
}




#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parser() {
        let parser = Parser::new("1+2").unwrap();
        assert_eq!(parser.current_token,Token::Num(1.0));
    }

    #[test]
    fn test_get_next_token() {
        let mut parser = Parser::new("1+2").unwrap();
        assert_eq!(parser.current_token,Token::Num(1.0));
        parser.get_next_token().unwrap();
        assert_eq!(parser.current_token,Token::Add);
        parser.get_next_token().unwrap();
        assert_eq!(parser.current_token,Token::Num(2.0));
    }

}