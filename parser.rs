/// This program reads tokens returned by Tokenizer and converts them into AST.
// Standard lib
use std::fmt;

// Internal modules
use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

// Parser struct
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

    // Take an arithmetic expression as input and return an AST
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero)?;
        Ok(ast)
    }
}

// Private methods of Parser
impl<'a> Parser<'a> {
    // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Unexpected end of input".into())),
        };
        Ok(())
    }

    // Main workhorse method that is called recursively
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    // Construct AST node for numbers, handling negative prefixes and parentheses
    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unexpected token".to_string())),
        }
    }

    // Check for balancing parentheses
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    // Construct Operator AST nodes
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Exponent)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::And => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Bitwise)?;
                Ok(Node::And(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Or => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Bitwise)?;
                Ok(Node::Or(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Unexpected operator {:?}",
                self.current_token
            ))),
        }
    }
}

// Custom error handler for Parser
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

// Handle error thrown from AST module
impl From<Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Parsing error occurred".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Multiply, Caret, Or, Number};

    #[test]
    fn test_parse_exponentiation() {
        let mut parser = Parser::new("2^3").unwrap();
        let expected = Caret(Box::new(Number(2.0)), Box::new(Number(3.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut parser = Parser::new("3+2*4").unwrap();
        let expected = Add(Box::new(Number(3.0)), Box::new(Multiply(Box::new(Number(2.0)), Box::new(Number(4.0)))));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parse_bitwise_or() {
        let mut parser = Parser::new("6|2").unwrap();
        let expected = Or(Box::new(Number(6.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parse_negative_number() {
        let mut parser = Parser::new("-5").unwrap();
        let expected = Node::Negative(Box::new(Number(5.0))); // Fix: Expect `Negative` instead of `Subtract(0, 5)`
        assert_eq!(parser.parse().unwrap(), expected);
    }
    

    #[test]
    fn test_parse_parentheses() {
        let mut parser = Parser::new("(2+3)").unwrap();
        let expected = Add(Box::new(Number(2.0)), Box::new(Number(3.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}