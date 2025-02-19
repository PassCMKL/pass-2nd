/// This contains the enum for the list of Tokens and handles Operator precedence rules.

// List of valid tokens that can be constructed from an arithmetic expression by the Tokenizer.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    And,        // &
    Or,         // |
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Caret,      // ^
    LeftParen,  // (
    RightParen, // )
    Num(f64),   // 12.34
    EOF,        // End of input
}

// Order of operators as per operator precedence rules (low to high)

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines all the `OperPrec` levels, from lowest to highest.
pub enum OperPrec {
    DefaultZero, // Default level (e.g., numbers)
    Bitwise,     // & and |
    AddSub,      // + and -
    MulDiv,      // * and /
    Exponent,    // ^
    Negative,    // Unary minus (-x)
}

// This contains methods to retrieve operator precedence for a given arithmetic operator

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            And | Or => Bitwise, // Bitwise operations have the lowest precedence
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Exponent,
            _ => DefaultZero, // Default case (numbers, EOF)
        }
    }
}