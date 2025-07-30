use crate::{
    lex::token::{Keyword, Operator, Token},
    parser::{
        ParserContext,
        ast::{Expression, Statement, Variable},
        block::BlockParser,
        expression::ExpressionParser,
        functions::TypeInfo,
        parser_error::ParserError,
        token_stream::TokenStream,
    },
};

pub struct StatementParser;

impl StatementParser {
    pub fn parse(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Statement, ParserError> {
        let mut window: [Option<Token>; 5] = [const { None }; 5];
        (0..5).for_each(|i| {
            window[i] = stream.peek_ahead(i).cloned();
        });

        match window {
            [Some(Token::OpenBrace), ..] => {
                let block = BlockParser::parse(ctx, stream)?;
                Ok(Statement::Block(block))
            }
            [
                Some(Token::Identifier(name)),
                Some(Token::Operator(Operator::Reassign)),
                ..,
            ] => {
                stream.advance(2)?;
                Ok(Statement::Expression(Expression::Variable(Variable {
                    name: name.clone(),
                    is_decl: false,
                    expression: Some(Box::new(ExpressionParser::parse(ctx, stream)?)),
                    type_info: None,
                })))
            }
            [
                Some(Token::Identifier(name)),
                Some(Token::Operator(Operator::Assign)),
                Some(Token::Operator(Operator::Reassign)),
                ..,
            ] => {
                stream.advance(3)?;
                Ok(Statement::Expression(Expression::Variable(Variable {
                    name: name.clone(),
                    is_decl: true,
                    expression: Some(Box::new(ExpressionParser::parse(ctx, stream)?)),
                    type_info: Some(TypeInfo {
                        name: None,
                        is_mut: false,
                    }),
                })))
            }
            [
                Some(Token::Identifier(name)),
                Some(Token::Operator(Operator::Assign)),
                Some(Token::Keyword(Keyword::Mut)),
                Some(Token::Operator(Operator::Reassign)),
                ..,
            ] => {
                stream.advance(4)?;
                Ok(Statement::Expression(Expression::Variable(Variable {
                    name: name.clone(),
                    is_decl: true,
                    expression: Some(Box::new(ExpressionParser::parse(ctx, stream)?)),
                    type_info: Some(TypeInfo {
                        name: None,
                        is_mut: true,
                    }),
                })))
            }
            // TODO: Support specifiying the type, either like:
            // foo :u32 = 10
            // foo :mut u32 = 11
            [Some(token), ..] => Err(ParserError::UnexpectedToken(token)),
            [None, ..] => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
