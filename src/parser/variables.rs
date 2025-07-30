use crate::{
    lex::token::{Operator, Token},
    parser::{
        ParserContext,
        ast::{Expression, Variable},
        expression::ExpressionParser,
        parser_error::ParserError,
    },
};
use std::fmt;

pub struct VariableParser;

impl VariableParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Variable, usize), ParserError> {
        if tokens.len() < 3 {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        if tokens[1] != Token::Operator(Operator::Assign) {
            return Err(ParserError::UnexpectedToken(tokens[1].clone()));
        }

        let name = tokens[0].clone();

        // TODO: Properly implement the type_info parsing logic
        // TODO: This is fucking discusting
        let (variable, token_length) = match tokens {
            [
                Token::Identifier(_),
                Token::Operator(Operator::Assign),
                Token::Operator(Operator::Reassign),
                rest @ ..,
            ] => {
                let (expression, token_length) = ExpressionParser::parse(ctx, rest)?;
                (
                    Variable {
                        name,
                        is_decl: true,
                        is_mut: false,
                        expression: Box::new(expression),
                    },
                    token_length + 3,
                )
            }
            [
                Token::Identifier(_),
                Token::Operator(Operator::Reassign),
                rest @ ..,
            ] => {
                // TODO: Validate that the variable has already been declared
                let (expression, token_length) = ExpressionParser::parse(ctx, rest)?;

                (
                    Variable {
                        name,
                        is_decl: false,
                        is_mut: false,
                        expression: Box::new(expression),
                    },
                    token_length + 2,
                )
            }
            _ => {
                return Err(ParserError::UnexpectedToken(tokens[2].clone()));
            }
        };
        Ok((variable, token_length))
    }
}
