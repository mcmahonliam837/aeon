use crate::{
    lex::token::Token,
    parser::{
        ParserContext, ast::Expression, parser_error::ParserError, token_stream::TokenStream,
        variables::VariableParser,
    },
};

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn parse(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<(Expression, usize), ParserError> {
        let start_position = stream.position();

        // LITERALS
        if let Some(Token::Literal(literal)) = stream.peek() {
            let literal = literal.clone();
            stream.advance(1)?;

            // Optional newline after literal
            stream.try_consume(Token::Newline);

            let end_position = stream.position();
            return Ok((Expression::Literal(literal), end_position - start_position));
        }

        // Try to parse as variable
        let checkpoint = stream.checkpoint();
        let variable_result = {
            let mut fork = stream.fork();
            VariableParser::parse(ctx, &mut fork)
        };

        match variable_result {
            Ok((variable, consumed)) => {
                // Advance the main stream
                stream.advance(consumed)?;

                let end_position = stream.position();
                Ok((
                    Expression::Variable(variable),
                    end_position - start_position,
                ))
            }
            Err(error) => {
                stream.restore(checkpoint);
                Err(error)
            }
        }
    }
}
