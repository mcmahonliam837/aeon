use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Statement, expression::ExpressionParser, parser_error::ParserError,
    },
};
pub struct StatementParser;

impl StatementParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Statement, usize), ParserError> {
        match tokens {
            tokens if !tokens.is_empty() => {
                let (expression, token_length) = ExpressionParser::parse(ctx, tokens)?;
                Ok((Statement::Expression(expression), token_length))
            }
            _ => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
