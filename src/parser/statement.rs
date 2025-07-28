use crate::{
    lex::token::{Operator, Token},
    parser::{
        ParserContext, ast::Statement, expression::ExpressionParser, parser_error::ParserError,
        variables::VariableParser,
    },
};
pub struct StatementParser;

impl StatementParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Statement, usize), ParserError> {
        match tokens {
            [Token::Identifier(_), Token::Operator(Operator::Assign), ..] => {
                let (variable, token_length) = VariableParser::parse(ctx, tokens)?;
                Ok((Statement::Variable(variable), token_length))
            }
            tokens if !tokens.is_empty() => {
                let (expression, token_length) = ExpressionParser::parse(ctx, tokens)?;
                Ok((Statement::Expression(expression), token_length))
            }
            _ => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
