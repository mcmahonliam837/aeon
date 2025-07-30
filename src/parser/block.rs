use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Statement, parser_error::ParserError, statement::StatementParser,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

pub struct BlockParser;

impl BlockParser {
    pub fn parse(ctx: &mut ParserContext, tokens: &[Token]) -> Result<(Block, usize), ParserError> {
        if tokens.is_empty() {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        if !matches!(tokens[0], Token::OpenBrace) {
            return Err(ParserError::UnexpectedToken(tokens[0].clone()));
        }

        let mut index = 0;

        let mut statements = Vec::new();

        while index < tokens.len() && !matches!(tokens[index], Token::CloseBrace) {
            let (statement, token_length) = StatementParser::parse(ctx, &tokens[index..])?;
            statements.push(statement);
            index += token_length;
        }

        Ok((Block { statements }, index + 1))
    }
}
