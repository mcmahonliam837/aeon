use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Statement, block::BlockParser, expression::ExpressionParser,
        parser_error::ParserError, token_stream::TokenStream,
    },
};

pub struct StatementParser;

impl StatementParser {
    pub fn parse(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<(Statement, usize), ParserError> {
        let start_position = stream.position();

        match stream.peek() {
            Some(Token::OpenBrace) => {
                let mut fork = stream.fork();
                let (block, consumed) = BlockParser::parse(ctx, &mut fork)?;

                stream.advance(consumed)?;

                let end_position = stream.position();
                Ok((Statement::Block(block), end_position - start_position))
            }
            Some(_) => {
                let mut fork = stream.fork();
                let (expression, consumed) = ExpressionParser::parse(ctx, &mut fork)?;

                stream.advance(consumed)?;

                let end_position = stream.position();
                Ok((
                    Statement::Expression(expression),
                    end_position - start_position,
                ))
            }
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
