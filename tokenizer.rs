/// This module reads characters in an arithmetic expression and converts them to tokens.
/// The allowed tokens are defined in the token module.

// Standard lib
use std::iter::Peekable;
use std::str::Chars;

// Internal modules
use super::token::Token;

// Tokenizer struct contains a Peekable iterator on the arithmetic expression
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }

    // Helper function to parse a number (integer or floating point)
    fn parse_number(&mut self, first_digit: char) -> Option<Token> {
        let mut num_str = first_digit.to_string();

        while let Some(&next) = self.expr.peek() {
            if next.is_ascii_digit() || next == '.' {
                num_str.push(self.expr.next().unwrap());
            } else {
                break;
            }
        }

        match num_str.parse::<f64>() {
            Ok(value) => Some(Token::Num(value)),
            Err(_) => None, // If parsing fails, return None
        }
    }
}

// Implement Iterator trait for Tokenizer struct.
// With this, we can use `next()` method on tokenizer to retrieve the next token from an arithmetic expression.

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(&c) = self.expr.peek() {
            match c {
                '0'..='9' => {
                    self.expr.next(); // Consume the character
                    return self.parse_number(c);
                }
                '+' => {
                    self.expr.next();
                    return Some(Token::Add);
                }
                '-' => {
                    self.expr.next();
                    return Some(Token::Subtract);
                }
                '*' => {
                    self.expr.next();
                    return Some(Token::Multiply);
                }
                '/' => {
                    self.expr.next();
                    return Some(Token::Divide);
                }
                '^' => {
                    self.expr.next();
                    return Some(Token::Caret);
                }
                '&' => {
                    self.expr.next();
                    return Some(Token::And);
                }
                '|' => {
                    self.expr.next();
                    return Some(Token::Or);
                }
                '(' => {
                    self.expr.next();
                    return Some(Token::LeftParen);
                }
                ')' => {
                    self.expr.next();
                    return Some(Token::RightParen);
                }
                ' ' | '\t' | '\n' => {
                    // Skip whitespace
                    self.expr.next();
                }
                _ => {
                    // If an unknown character is found, return None
                    self.expr.next();
                    return None;
                }
            }
        }
        Some(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_exponentiation() {
        let mut tokenizer = Tokenizer::new("2^3");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Caret);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
    }

    #[test]
    fn test_tokenize_mixed_expression() {
        let mut tokenizer = Tokenizer::new("10+2*3/4-5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(10.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Multiply);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Divide);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(4.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Subtract);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(5.0));
    }

    #[test]
    fn test_tokenize_bitwise_and_or() {
        let mut tokenizer = Tokenizer::new("6&3|2");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(6.0));
        assert_eq!(tokenizer.next().unwrap(), Token::And);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(3.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Or);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut tokenizer = Tokenizer::new("-5");
        assert_eq!(tokenizer.next().unwrap(), Token::Subtract);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(5.0));
    }

    #[test]
    fn test_tokenize_parentheses() {
        let mut tokenizer = Tokenizer::new("(1+2)");
        assert_eq!(tokenizer.next().unwrap(), Token::LeftParen);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(1.0));
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Num(2.0));
        assert_eq!(tokenizer.next().unwrap(), Token::RightParen);
    }
}