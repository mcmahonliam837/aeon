use crate::{
    lex::token::Token,
    parser::{
        ParserContext,
        ast::{Expression, Variable},
        parser_error::ParserError,
        token_stream::TokenStream,
    },
};

pub struct ExpressionParser;

impl ExpressionParser {
    pub fn parse(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let result = Self::equality(ctx, stream);
        stream.try_consume(Token::Newline);
        result
    }

    fn equality(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let mut expr = Self::comparison(ctx, stream)?;
        while stream.current()?.is_equality() {
            stream.advance(1)?;
            let Token::Operator(operator) = stream.previous()?.clone() else {
                return Err(ParserError::UnexpectedToken(stream.previous()?.clone()));
            };

            let right = Self::comparison(ctx, stream)?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let mut expr = Self::term(ctx, stream)?;
        while stream.current()?.is_comparison() {
            stream.advance(1)?;
            let Token::Operator(operator) = stream.previous()?.clone() else {
                return Err(ParserError::UnexpectedToken(stream.previous()?.clone()));
            };

            let right = Self::term(ctx, stream)?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(ctx: &mut ParserContext, stream: &mut TokenStream) -> Result<Expression, ParserError> {
        let mut expr = Self::factor(ctx, stream)?;
        while stream.current()?.is_term() {
            stream.advance(1)?;
            let Token::Operator(operator) = stream.previous()?.clone() else {
                return Err(ParserError::UnexpectedToken(stream.previous()?.clone()));
            };

            let right = Self::factor(ctx, stream)?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let mut expr = Self::unary(ctx, stream)?;
        while stream.current()?.is_factor() {
            stream.advance(1)?;
            let Token::Operator(operator) = stream.previous()?.clone() else {
                return Err(ParserError::UnexpectedToken(stream.previous()?.clone()));
            };

            let right = Self::unary(ctx, stream)?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(ctx: &mut ParserContext, stream: &mut TokenStream) -> Result<Expression, ParserError> {
        if stream.current()?.is_unary() {
            stream.advance(1)?;
            let Token::Operator(operator) = stream.previous()?.clone() else {
                return Err(ParserError::UnexpectedToken(stream.previous()?.clone()));
            };

            return Ok(Expression::Unary {
                operator,
                operand: Box::new(Self::unary(ctx, stream)?),
            });
        }

        Self::primary(ctx, stream)
    }

    fn primary(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let token = stream.current()?.clone();
        stream.advance(1)?;
        match token {
            Token::OpenParen => Self::grouped(ctx, stream),
            Token::Literal(literal) => Ok(Expression::Literal(literal.clone())),
            Token::Identifier(name) => Ok(Expression::Variable(Variable {
                name: name.clone(),
                is_decl: false,
                expression: None,
                type_info: None,
            })),
            _ => Err(ParserError::UnexpectedToken(token.clone())),
        }
    }

    fn grouped(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<Expression, ParserError> {
        let expr = Self::parse(ctx, stream)?;
        stream.consume(Token::CloseParen)?;
        Ok(Expression::Group {
            inner: Box::new(expr),
        })
    }
}
