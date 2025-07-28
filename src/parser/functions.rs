use crate::{
    lex::token::{Keyword, Token},
    parser::{ParserError, ast::Statement, parse_statement, variables::Variable},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub name: Token,
    pub type_: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decl: Token,
    pub name: Token,
    pub parameters: Vec<Arg>,
    pub statements: Vec<Statement>,
}

pub fn parse_function(tokens: &[Token]) -> Result<(Function, usize), ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if !matches!(
        tokens[..3],
        [
            Token::Keyword(Keyword::Fn),
            Token::Identifier(_),
            Token::OpenParen
        ]
    ) {
        return Err(ParserError::UnexpectedToken(tokens[0].clone()));
    }

    let mut index = 0;

    let decl = tokens[index].clone();
    index += 1;

    let name = tokens[index].clone();
    index += 2; // Advance 2 tokens to skip the identifier and open parenthesis

    let mut parameters = Vec::new();
    while !matches!(tokens[index], Token::CloseParen) {
        let (arg, token_length) = parse_arg(&tokens[index..])?;
        parameters.push(arg);
        index += token_length;
    }

    let (statements, block_length) = parse_block(&tokens[index..])?;

    Ok((
        Function {
            decl,
            name,
            parameters,
            statements,
        },
        index + block_length,
    ))
}

fn parse_arg(tokens: &[Token]) -> Result<(Arg, usize), ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if !matches!(tokens[0], Token::Identifier(_)) {
        return Err(ParserError::UnexpectedToken(tokens[0].clone()));
    }

    let mut index = 0;

    let name = tokens[index].clone();
    index += 1;

    if !matches!(tokens[index], Token::Identifier(_)) {
        return Err(ParserError::UnexpectedToken(tokens[index].clone()));
    }

    let type_ = tokens[index].clone();
    index += 1;

    if !matches!(tokens[index], Token::Comma) || !matches!(tokens[index], Token::CloseParen) {
        return Err(ParserError::UnexpectedToken(tokens[index].clone()));
    }
    index += 1;

    Ok((Arg { name, type_ }, index))
}

fn parse_block(tokens: &[Token]) -> Result<(Vec<Statement>, usize), ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if !matches!(tokens[0], Token::OpenBrace) {
        return Err(ParserError::UnexpectedToken(tokens[0].clone()));
    }

    let mut index = 1;

    let mut statements = Vec::new();

    while !matches!(tokens[index], Token::CloseBrace) {
        let (statement, token_length) = parse_statement(&tokens[index..])?;
        statements.push(statement);
        index += token_length;
    }

    Ok((statements, index + 1))
}
