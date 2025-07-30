use crate::{
    lex::token::{Keyword, Token},
    parser::{
        ParserContext, ParserError,
        block::{Block, BlockParser},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub name: Token,
    pub type_info: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decl: Token,
    pub name: String,
    pub parameters: Vec<Arg>,
    pub return_type: TypeInfo,
    pub block: Block,
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

        let name = match &tokens[index] {
            Token::Identifier(name) => {
                ctx.enter_function(name.clone());
                name.clone()
            }
            _ => {
                return Err(ParserError::UnexpectedToken(tokens[index].clone()));
            }
        };

        index += 2; // Advance 2 tokens to skip the identifier and open parenthesis

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

        // TODO: This is hacky. Update the parser to convert keywords
        // and identifiers to a Type enum.
        let return_type = match &tokens[index] {
            Token::Identifier(name) => {
                index += 1;
                TypeInfo { name: name.clone() }
            }
            Token::OpenBrace => TypeInfo {
                name: "void".to_string(),
            },
            _ => {
                return Err(ParserError::UnexpectedToken(tokens[index].clone()));
            }
        };

        let (block, block_length) = BlockParser::parse(ctx, &tokens[index..])?;

        Ok((
            Function {
                decl,
                name,
                parameters,
                return_type,
                block,
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

    let type_info = match &tokens[index] {
        Token::Identifier(type_name) => TypeInfo {
            name: type_name.clone(),
        },
        _ => return Err(ParserError::UnexpectedToken(tokens[index].clone())),
    };

    index += 1;

    if !matches!(tokens[index], Token::Comma) && !matches!(tokens[index], Token::CloseParen) {
        return Err(ParserError::UnexpectedToken(tokens[index].clone()));
    }

    // Don't consume the delimiter - let the caller handle it
    Ok((Arg { name, type_info }, index))
}
