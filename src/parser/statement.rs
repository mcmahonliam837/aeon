use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Statement, block::BlockParser, expression::ExpressionParser,
        parser_error::ParserError,
    },
};
pub struct StatementParser;

impl StatementParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Statement, usize), ParserError> {
        match tokens {
            [Token::OpenBrace, ..] => {
                let (block, token_length) = BlockParser::parse(ctx, tokens)?;
                Ok((Statement::Block(block), token_length))
            }
            tokens if !tokens.is_empty() => {
                let (expression, token_length) = ExpressionParser::parse(ctx, tokens)?;
                Ok((Statement::Expression(expression), token_length))
            }
            _ => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
