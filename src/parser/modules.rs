use crate::{
    lex::token::{Keyword, Literal, Token},
    parser::{
        ParserContext,
        ast::Variable,
        functions::{Function, FunctionParser},
        parser_error::ParserError,
        token_stream::{self, TokenStream},
        variables::VariableParser,
    },
};

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

        while !stream.is_at_end() {
            match stream.peek() {
                Some(Token::Keyword(Keyword::Module)) => {
                    let decl = stream.consume_exact(Token::Keyword(Keyword::Module))?;

                    let name = match stream.consume(Token::Identifier(String::new()))? {
                        Token::Identifier(n) => {
                            ctx.enter_module(n.clone());
                            n
                        }
                        token => return Err(ParserError::UnexpectedToken(token)),
                    };

                    stream.consume(Token::OpenBrace)?;
                    _ = stream.try_consume(Token::Newline);

                    // Find matching closing brace
                    let mut brace_level = 1;
                    let body_start = stream.position();

                    while !stream.is_at_end() && brace_level > 0 {
                        let token = stream.current()?;
                        match token {
                            Token::OpenBrace => {
                                brace_level += 1;
                                stream.advance(1)?;
                            }
                            Token::CloseBrace => {
                                brace_level -= 1;
                                stream.advance(1)?;
                                if brace_level == 0 {
                                    break;
                                }
                            }
                            _ => stream.advance(1)?,
                        }
                    }

                    if brace_level > 0 {
                        return Err(ParserError::MissingClosingBrace {
                            start: decl,
                            end: None,
                        });
                    }

                    // Parse the body with a new stream
                    // -2 due to the stream having advanced to the closing brace
                    // then to the next token
                    let mut body_stream = stream.substream(body_start, stream.position() - 2);
                    let (inner_imports, inner_modules, inner_functions, inner_variables) =
                        Self::parse_module_body(ctx, &mut body_stream)?;

                    modules.push(Module {
                        decl,
                        name,
                        imports: inner_imports,
                        modules: inner_modules,
                        functions: inner_functions,
                        variables: inner_variables,
                    });

                    ctx.exit_module();
                    continue;
                }
                Some(Token::Identifier(_)) => {
                    let mut temp_stream = stream.fork();
                    match VariableParser::parse(ctx, &mut temp_stream) {
                        Ok((variable, consumed)) => {
                            variables.push(variable);
                            stream.advance(consumed)?;
                            stream.try_consume(Token::Newline);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                    // TODO: Match for function calls
                    continue;
                }
                Some(Token::Keyword(Keyword::Fn)) => {
                    let mut temp_stream = stream.fork();
                    let (function, consumed) = FunctionParser::parse(ctx, &mut temp_stream)?;
                    functions.push(function);
                    stream.advance(consumed)?;
                    stream.try_consume(Token::Newline);
                    ctx.exit_function();
                    continue;
                }
                Some(Token::Keyword(Keyword::Import)) => {
                    let decl = stream.consume(Token::Keyword(Keyword::Import))?;

                    let Token::Literal(Literal::String(path)) =
                        stream.consume(Token::Literal(Literal::String(String::new())))?
                    else {
                        return Err(ParserError::UnexpectedToken(stream.current()?.clone()));
                    };

                    imports.push(Import { path, decl });

                    continue;
                }
                Some(_) => stream.advance(1)?,
                None => break,
            }
        }

        Ok((imports, modules, functions, variables))
    }
}
