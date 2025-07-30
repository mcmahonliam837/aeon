use crate::{
    lex::token::{Keyword, Token},
    parser::{ParserContext, ParserError, ast::Statement, statement::StatementParser},
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
    pub return_type: Token,
    pub statements: Vec<Statement>,
}

pub struct FunctionParser;
impl FunctionParser {
    pub fn parse(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Function, usize), ParserError> {
        if tokens.len() < 3 {
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

        match name {
            Token::Identifier(ref name) => {
                ctx.enter_function(name.clone());
            }
            _ => {
                return Err(ParserError::UnexpectedToken(name));
            }
        }

        let mut parameters = Vec::new();
        while !matches!(tokens[index], Token::OpenBrace)
            && !matches!(tokens[index], Token::CloseParen)
        {
            let (arg, token_length) = parse_arg(&tokens[index..])?;
            parameters.push(arg);
            index += token_length;

            // Consume comma if present
            if matches!(tokens[index], Token::Comma) {
                index += 1;
            }
        }

        if tokens[index] == Token::CloseParen {
            index += 1;
        }

        // Parse the return type
        if tokens.len() <= index {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        if !matches!(tokens[index], Token::Identifier(_)) {
            return Err(ParserError::UnexpectedToken(tokens[index].clone()));
        }

        let return_type = tokens[index].clone();
        index += 1;

        let (statements, block_length) = parse_block(ctx, &tokens[index..])?;

        Ok((
            Function {
                decl,
                name,
                parameters,
                return_type,
                statements,
            },
            index + block_length,
        ))
    }
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

    if !matches!(tokens[index], Token::Comma) && !matches!(tokens[index], Token::CloseParen) {
        return Err(ParserError::UnexpectedToken(tokens[index].clone()));
    }

    // Don't consume the delimiter - let the caller handle it
    Ok((Arg { name, type_ }, index))
}

fn parse_block(
    ctx: &mut ParserContext,
    tokens: &[Token],
) -> Result<(Vec<Statement>, usize), ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if !matches!(tokens[0], Token::OpenBrace) {
        return Err(ParserError::UnexpectedToken(tokens[0].clone()));
    }

    let mut index = 1;

    let mut statements = Vec::new();

    while index < tokens.len() && !matches!(tokens[index], Token::CloseBrace) {
        let (statement, token_length) = StatementParser::parse(ctx, &tokens[index..])?;
        statements.push(statement);
        index += token_length;
    }

    Ok((statements, index + 1))
}
