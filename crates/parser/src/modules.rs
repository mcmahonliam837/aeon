use crate::{
    ParserContext,
    ast::{Expression, Statement, Variable},
    functions::{Function, FunctionParser},
    parser_error::ParserError,
    statement::StatementParser,
    token_stream::TokenStream,
};

use lex::token::{Keyword, Literal, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub decl: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub decl: Token,
    pub name: String,
    pub imports: Vec<Import>,
    pub modules: Vec<Module>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

pub struct ModuleParser;

impl ModuleParser {
    pub fn parse(ctx: &mut ParserContext, stream: &mut TokenStream) -> Result<Module, ParserError> {
        let decl = stream
            .consume_exact(Token::Keyword(Keyword::Module))
            .or(Err(ParserError::ModuleNotFound))?;
        let name_token = stream
            .consume(Token::Identifier(String::new()))
            .or(Err(ParserError::ModuleWithoutName))?;
        let name = match name_token {
            Token::Identifier(name) => {
                ctx.enter_module(name.clone());
                name
            }
            _ => return Err(ParserError::ModuleWithoutName),
        };

        let (imports, modules, functions, variables) = Self::parse_module_body(ctx, stream)?;
        Ok(Module {
            decl,
            name,
            imports,
            modules,
            functions,
            variables,
        })
    }

    #[allow(clippy::type_complexity)]
    fn parse_module_body(
        ctx: &mut ParserContext,
        stream: &mut TokenStream,
    ) -> Result<(Vec<Import>, Vec<Module>, Vec<Function>, Vec<Variable>), ParserError> {
        let mut imports = Vec::<Import>::new();
        let mut modules = Vec::<Module>::new();
        let mut functions = Vec::<Function>::new();
        let mut variables = Vec::<Variable>::new();

        loop {
            if stream.is_at_end() {
                break;
            }

            let token = stream
                .peek()
                .cloned()
                .expect("Should have current token, since we are not at the end of the stream");

            match token {
                Token::Keyword(Keyword::Module) => {
                    let decl = stream.consume_exact(Token::Keyword(Keyword::Module))?;

                    let name = match stream.consume(Token::Identifier(String::new()))? {
                        Token::Identifier(n) => {
                            ctx.enter_module(n.clone());
                            ctx.get_fully_qualified_module_name()
                        }
                        token => return Err(ParserError::UnexpectedToken(token)),
                    };

                    stream.consume(Token::OpenBrace)?;
                    _ = stream.try_consume(Token::Newline);

                    // Parse the body with a new stream
                    // -2 due to the stream having advanced to the closing brace
                    // then to the next token
                    // let mut body_stream = stream.substream(body_start, stream.position() - 2);
                    let (inner_imports, inner_modules, inner_functions, inner_variables) =
                        Self::parse_module_body(ctx, stream)?;

                    modules.push(Module {
                        decl,
                        name,
                        imports: inner_imports,
                        modules: inner_modules,
                        functions: inner_functions,
                        variables: inner_variables,
                    });

                    continue;
                }
                token @ Token::Identifier(_) => {
                    match StatementParser::parse(ctx, stream)? {
                        Statement::Expression(Expression::Variable(variable)) => {
                            variables.push(variable);
                            stream.try_consume(Token::Newline);
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken(token.clone()));
                        }
                    }
                    // TODO: Match for function calls
                    continue;
                }
                Token::Keyword(Keyword::Fn) => {
                    let function = FunctionParser::parse(ctx, stream)?;
                    functions.push(function);
                    stream.try_consume(Token::Newline);
                    ctx.exit_function();
                    continue;
                }
                Token::Keyword(Keyword::Import) => {
                    let decl = stream.consume(Token::Keyword(Keyword::Import))?;

                    let Token::Literal(Literal::String(path)) =
                        stream.consume(Token::Literal(Literal::String(String::new())))?
                    else {
                        return Err(ParserError::UnexpectedToken(stream.current()?.clone()));
                    };

                    imports.push(Import { path, decl });

                    continue;
                }
                Token::CloseBrace => {
                    stream.advance(1)?;
                    break;
                }
                _ => stream.advance(1)?,
            }
        }

        ctx.exit_module();
        Ok((imports, modules, functions, variables))
    }
}
