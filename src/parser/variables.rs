use crate::{
    lex::token::{Operator, Token},
    parser::{ast::Expression, parse_expression, parser_error::ParserError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Token,
    pub expression: Expression,
}

pub fn parse_variable(tokens: &[Token]) -> Result<(Variable, usize), ParserError> {
    println!("Parsing variable from: {:?}", tokens);
    if tokens.len() < 3 {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if tokens[1] != Token::Operator(Operator::Assign) {
        return Err(ParserError::UnexpectedToken(tokens[1].clone()));
    }

    let name = tokens[0].clone();

    let (expression, token_length) = parse_expression(&tokens[2..])?;

    Ok((Variable { name, expression }, token_length))
}
