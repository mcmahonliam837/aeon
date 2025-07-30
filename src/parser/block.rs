use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Statement, parser_error::ParserError, statement::StatementParser,
        token_stream::TokenStream,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

pub struct BlockParser;

impl BlockParser {
    pub fn parse(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<(Block, usize), ParserError> {
        let start_position = stream.position();

        // Consume opening brace
        stream.consume(Token::OpenBrace)?;

        let mut statements = Vec::new();

        while !stream.is_at_end() {
            match stream.peek() {
                Some(Token::CloseBrace) => {
                    stream.advance(1)?;
                    break;
                }
                _ => {
                    let mut fork = stream.fork();
                    let (statement, consumed) = StatementParser::parse(ctx, &mut fork)?;
                    statements.push(statement);

                    stream.advance(consumed)?;

                    // Advance the main stream
                    while stream.position() < fork.position() {
                        stream.advance(1)?;
                    }
                }
            }
        }

        let end_position = stream.position();
        Ok((Block { statements }, end_position - start_position))
    }
}
