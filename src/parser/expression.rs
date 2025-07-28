use crate::{
    lex::token::Token,
    parser::{ParserContext, ast::Expression, parser_error::ParserError},
};

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn parse(
        _ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Expression, usize), ParserError> {
        println!("Parsing expression: {:?}", tokens);
        match tokens {
            [Token::Literal(literal), Token::Newline, ..] => {
                Ok((Expression::Literal(literal.clone()), 2))
            }
            [Token::Literal(literal)] => Ok((Expression::Literal(literal.clone()), 1)),
            [] => Err(ParserError::UnexpectedEndOfInput),
            _ => Err(ParserError::UnexpectedToken(tokens[0].clone())),
        }
    }
}
