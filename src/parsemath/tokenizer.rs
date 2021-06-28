

use std::iter::Peekable;
use std::str::Chars;
use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}


impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}


impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }

    #[test]
    fn test_operator_add_sign() {
        let mut tokenizer = Tokenizer::new("+");
        assert_eq!(tokenizer.next().unwrap(), Token::Add)
    }

    #[test]
    fn test_operator_mixed_chars() {
        let mut tokenizer = Tokenizer::new("3+4");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(4.0))

    }

    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    #[ignore]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5));
    }
}

