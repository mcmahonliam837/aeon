use crate::{
    lex::token::{Operator, Token},
    parser::{
        ParserContext, ast::Expression, expression::ExpressionParser, parser_error::ParserError,
    },
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Token,
    pub expression: Expression,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} := {}", self.name, self.expression)
    }
}

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

        let (expression, token_length) = ExpressionParser::parse(ctx, &tokens[2..])?;

        Ok((Variable { name, expression }, token_length + 2))
    }
}
