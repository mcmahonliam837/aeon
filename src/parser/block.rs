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
    pub fn parse(ctx: &mut ParserContext, stream: &mut TokenStream) -> Result<Block, ParserError> {
        // Consume opening brace
        stream.consume(Token::OpenBrace)?;
        stream.try_consume(Token::Newline);

        let mut statements = Vec::new();

        loop {
            match stream.peek() {
                Some(Token::CloseBrace) => {
                    stream.advance(1)?;
                    break;
                }
                Some(_) => {
                    let statement = StatementParser::parse(ctx, stream)?;
                    statements.push(statement);
                }
                None => break,
            }
        }

        Ok(Block { statements })
    }
}
