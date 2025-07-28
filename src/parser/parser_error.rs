use std::error::Error;

use crate::lex::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ModuleNotFound,
    ModuleWithoutName,
    ModuleEmpty { start: Token, end: Token },
    NestedModuleMustBeTopLevel { start: Token, end: Option<Token> },
    NestedModuleWithoutBody { start: Token, end: Token },
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    MissingClosingBrace { start: Token, end: Option<Token> },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
            ParserError::ModuleNotFound => {
                write!(f, "File must begin with a module declaration!")
            }
            ParserError::ModuleWithoutName => {
                write!(f, "Module declaration must have a name!")
            }
            // TODO: Tokens should store their position information
            // so that it can be used to provide more detailed error messages.
            ParserError::ModuleEmpty {
                start: _start,
                end: _end,
            } => {
                write!(f, "Empty module declaration!")
            }
            ParserError::NestedModuleMustBeTopLevel {
                start: _start,
                end: _end,
            } => {
                write!(f, "Nested module must be top-level!")
            }
            ParserError::NestedModuleWithoutBody {
                start: _start,
                end: _end,
            } => {
                write!(f, "Nested module must have a body!")
            }
            ParserError::MissingClosingBrace {
                start: _start,
                end: _end,
            } => {
                write!(f, "Missing closing brace!")
            }
            ParserError::UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input!")
            }
        }
    }
}

impl Error for ParserError {}
