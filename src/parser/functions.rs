use crate::{
    lex::token::{Keyword, Token},
    parser::{
        ParserContext, ParserError,
        block::{Block, BlockParser},
        token_stream::TokenStream,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub name: Token,
    pub type_info: TypeInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    pub name: Option<String>,
    pub is_mut: bool,
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
        stream: &mut TokenStream,
    ) -> Result<Function, ParserError> {
        let decl = stream.consume(Token::Keyword(Keyword::Fn))?;

        let name_token = stream.consume(Token::Identifier(String::new()))?;
        let name = match name_token {
            Token::Identifier(name) => {
                ctx.enter_function(name.clone());
                name
            }
            _ => {
                return Err(ParserError::UnexpectedToken(name_token));
            }
        };

        stream.consume(Token::OpenParen)?;

        let mut parameters = Vec::new();
        while !stream.is_at_end() {
            match stream.peek() {
                Some(Token::CloseParen) => {
                    stream.advance(1)?;
                    break;
                }
                Some(Token::OpenBrace) => break,
                _ => {
                    let arg = parse_arg(stream)?;
                    parameters.push(arg);

                    // Consume comma if present
                    stream.try_consume(Token::Comma);
                }
            }
        }

        // Parse the return type
        // TODO: This is hacky. Update the parser to convert keywords
        // and identifiers to a Type enum.
        let return_type = match stream.peek() {
            Some(Token::Identifier(_)) => {
                let type_token = stream.consume(Token::Identifier(String::new()))?;
                match type_token {
                    Token::Identifier(type_name) => TypeInfo {
                        name: Some(type_name),
                        is_mut: true,
                    },
                    _ => unreachable!(),
                }
            }
            Some(Token::OpenBrace) => TypeInfo {
                name: Some("void".to_string()),
                is_mut: false,
            },
            _ => {
                return Err(ParserError::UnexpectedToken(stream.current()?.clone()));
            }
        };

        let block = BlockParser::parse(ctx, stream)?;

        Ok(Function {
            decl,
            name,
            parameters,
            return_type,
            block,
        })
    }
}

fn parse_arg(stream: &mut TokenStream) -> Result<Arg, ParserError> {
    // Parse argument name
    let name = stream.consume(Token::Identifier(String::new()))?;

    // Parse argument type
    let type_token = stream.consume(Token::Identifier(String::new()))?;
    let type_info = match type_token {
        Token::Identifier(type_name) => TypeInfo {
            name: Some(type_name),
            is_mut: false,
        },
        _ => return Err(ParserError::UnexpectedToken(type_token)),
    };

    // Check that we have a valid delimiter after the argument
    match stream.peek() {
        Some(Token::Comma) | Some(Token::CloseParen) => {
            // Don't consume the delimiter - let the caller handle it
        }
        Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
        None => return Err(ParserError::UnexpectedEndOfInput),
    }

    Ok(Arg { name, type_info })
}
