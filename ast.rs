/// This program contains a list of valid AST nodes that can be constructed and also evaluates an AST to compute a value
// Standard lib
use std::error;

// List of allowed AST nodes that can be constructed by the Parser
// Tokens can be arithmetic operators or a Number
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // WARNING: Bitwise And and Or operations only work on integer values
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),

    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

// Given an AST, calculate the numeric value.
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => {
            let divisor = eval(*expr2)?;
            if divisor == 0.0 {
                return Err("Division by zero".into());
            }
            Ok(eval(*expr1)? / divisor)
        }
        Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Negative(expr) => Ok(-eval(*expr)?),
        And(expr1, expr2) => {
            let left = eval(*expr1)? as i64;
            let right = eval(*expr2)? as i64;
            Ok((left & right) as f64)
        }
        Or(expr1, expr2) => {
            let left = eval(*expr1)? as i64;
            let right = eval(*expr2)? as i64;
            Ok((left | right) as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Multiply, Divide, Caret, And, Or, Number};

    #[test]
    fn test_exponentiation() {
        let expr = Caret(Box::new(Number(2.0)), Box::new(Number(3.0)));
        assert_eq!(eval(expr).unwrap(), 8.0);
    }

    #[test]
    fn test_bitwise_and() {
        let expr = And(Box::new(Number(6.0)), Box::new(Number(3.0))); // 6 & 3 = 2
        assert_eq!(eval(expr).unwrap(), 2.0);
    }

    #[test]
    fn test_bitwise_or() {
        let expr = Or(Box::new(Number(6.0)), Box::new(Number(3.0))); // 6 | 3 = 7
        assert_eq!(eval(expr).unwrap(), 7.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Divide(Box::new(Number(5.0)), Box::new(Number(0.0)));
        assert!(eval(expr).is_err());
    }

    #[test]
    fn test_nested_expression() {
        let expr = Add(
            Box::new(Number(3.0)),
            Box::new(Multiply(Box::new(Number(2.0)), Box::new(Number(5.0)))),
        ); // 3 + (2 * 5) = 13
        assert_eq!(eval(expr).unwrap(), 13.0);
    }
}