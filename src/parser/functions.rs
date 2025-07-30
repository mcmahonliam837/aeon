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
        stream: &mut TokenStream,
    ) -> Result<(Function, usize), ParserError> {
        let start_position = stream.position();

        // Consume 'fn' keyword
        let decl = stream.consume(Token::Keyword(Keyword::Fn))?;

        // Consume function name
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

        // Consume opening parenthesis
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
                    let mut fork = stream.fork();
                    let (arg, consumed) = parse_arg(&mut fork)?;
                    parameters.push(arg);

                    stream.advance(consumed)?;

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
                    Token::Identifier(type_name) => TypeInfo { name: type_name },
                    _ => unreachable!(),
                }
            }
            Some(Token::OpenBrace) => TypeInfo {
                name: "void".to_string(),
            },
            _ => {
                return Err(ParserError::UnexpectedToken(stream.current()?.clone()));
            }
        };

        let mut fork = stream.fork();
        let (block, consumed) = BlockParser::parse(ctx, &mut fork)?;

        stream.advance(consumed)?;

        let end_position = stream.position();

        Ok((
            Function {
                decl,
                name,
                parameters,
                return_type,
                block,
            },
            end_position - start_position,
        ))
    }
}

fn parse_arg(stream: &mut TokenStream) -> Result<(Arg, usize), ParserError> {
    let start_position = stream.position();

    // Parse argument name
    let name = stream.consume(Token::Identifier(String::new()))?;

    // Parse argument type
    let type_token = stream.consume(Token::Identifier(String::new()))?;
    let type_info = match type_token {
        Token::Identifier(type_name) => TypeInfo { name: type_name },
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

    let end_position = stream.position();
    Ok((Arg { name, type_info }, end_position - start_position))
}
