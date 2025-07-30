use crate::{
    lex::token::{Operator, Token},
    parser::{
        ParserContext, ast::Expression, parser_error::ParserError, variables::VariableParser,
    },
};

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Expression, usize), ParserError> {
        if tokens.is_empty() {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        // LITERALS
        match tokens {
            [Token::Literal(literal), Token::Newline, ..] => {
                return Ok((Expression::Literal(literal.clone()), 2));
            }
            [Token::Literal(literal)] => return Ok((Expression::Literal(literal.clone()), 1)),
            _ => {}
        }

        let variable_error = match VariableParser::parse(ctx, tokens) {
            Ok((variable, token_length)) => {
                return Ok((Expression::Variable(variable), token_length));
            }
            Err(error) => Some(error),
        };

        if let Some(error) = variable_error {
            return Err(error);
        }
        Err(ParserError::UnexpectedToken(tokens[0].clone()))
    }
}
