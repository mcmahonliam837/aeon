use crate::{
    lex::token::{Keyword, Literal, Token},
    parser::{
        ParserContext,
        functions::{Function, FunctionParser},
        parser_error::ParserError,
        variables::{Variable, VariableParser},
    },
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub decl: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Token,
    pub imports: Vec<Import>,
    pub modules: Vec<Module>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module {}", self.name)?;

        if !self.variables.is_empty() || !self.functions.is_empty() || !self.modules.is_empty() {
            write!(f, " {{")?;

            // Display variables
            for var in &self.variables {
                write!(f, "\n  {}", var)?;
            }

            // Display functions
            for func in &self.functions {
                write!(f, "\n  fn {}(...)", func.name)?;
            }

            // Display nested modules
            for module in &self.modules {
                write!(f, "\n  module {} {{...}}", module.name)?;
            }

            write!(f, "\n}}")?;
        }

        Ok(())
    }
}

pub struct ModuleParser;

impl ModuleParser {
    pub fn parse(ctx: &mut ParserContext, tokens: &[Token]) -> Result<Module, ParserError> {
        Self::validate_module_decl(tokens)?;
        let name: Token = tokens[1].clone();
        match name {
            Token::Identifier(ref name) => ctx.enter_module(name.clone()),
            _ => return Err(ParserError::ModuleWithoutName),
        }

        let (imports, modules, functions, variables) = Self::parse_module_body(ctx, &tokens[2..])?;
        Ok(Module {
            name,
            imports,
            modules,
            functions,
            variables,
        })
    }

    fn validate_module_decl(tokens: &[Token]) -> Result<(), ParserError> {
        if tokens.is_empty() {
            return Err(ParserError::ModuleNotFound);
        }
        if !matches!(tokens[0], Token::Keyword(Keyword::Module)) {
            return Err(ParserError::ModuleNotFound);
        }
        if tokens.len() < 2 || !matches!(tokens[1], Token::Identifier(_)) {
            return Err(ParserError::ModuleWithoutName);
        }
        if tokens.len() == 2 {
            return Err(ParserError::ModuleEmpty {
                start: tokens[0].clone(),
                end: tokens[1].clone(),
            });
        }

        Ok(())
    }

    fn validate_nested_module_decl(tokens: &[Token]) -> Result<(), ParserError> {
        Self::validate_module_decl(tokens)?;

        if tokens.len() < 3 {
            return Err(ParserError::ModuleEmpty {
                start: tokens[0].clone(),
                end: tokens[2].clone(),
            });
        }

        if !matches!(tokens[2], Token::OpenBrace) {
            return Err(ParserError::ModuleEmpty {
                start: tokens[0].clone(),
                end: tokens[2].clone(),
            });
        }

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn parse_module_body(
        ctx: &mut ParserContext,
        tokens: &[Token],
    ) -> Result<(Vec<Import>, Vec<Module>, Vec<Function>, Vec<Variable>), ParserError> {
        let mut imports = Vec::<Import>::new();
        let mut modules = Vec::<Module>::new();
        let mut functions = Vec::<Function>::new();
        let mut variables = Vec::<Variable>::new();

        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            match token {
                Token::Keyword(Keyword::Module) => {
                    let tokens = &tokens[index..];
                    Self::validate_nested_module_decl(&tokens[..3])?;
                    let mut brace_level = 0;
                    let Some(body_token_length) = tokens.iter().position(|t| match t {
                        Token::OpenBrace => {
                            brace_level += 1;
                            false
                        }
                        Token::CloseBrace => {
                            brace_level -= 1;
                            brace_level == 0
                        }
                        _ => false,
                    }) else {
                        return Err(ParserError::MissingClosingBrace {
                            start: tokens[2].clone(),
                            end: None, // TODO: Find the actual end of this module decl
                        });
                    };
                    let (inner_imports, inner_modules, inner_functions, inner_variables) =
                        Self::parse_module_body(ctx, &tokens[3..body_token_length - 1])?;

                    modules.push(Module {
                        name: tokens[1].clone(),
                        imports: inner_imports,
                        modules: inner_modules,
                        functions: inner_functions,
                        variables: inner_variables,
                    });

                    index += body_token_length + 1;
                    ctx.exit_module();
                    continue;
                }
                Token::Identifier(_) => {
                    let tokens = &tokens[index..];

                    let (variable, token_length) = VariableParser::parse(ctx, tokens)?;

                    variables.push(variable);

                    index += token_length;

                    // Skip newline if present
                    if index < tokens.len() && matches!(tokens[index], Token::Newline) {
                        index += 1;
                    }
                    continue;
                }
                Token::Keyword(Keyword::Fn) => {
                    let tokens = &tokens[index..];

                    let (function, token_length) = FunctionParser::parse(ctx, tokens)?;

                    functions.push(function);

                    index += token_length + 1;

                    ctx.exit_function();
                    continue;
                }
                Token::Keyword(Keyword::Import) => {
                    let tokens = &tokens[index..];

                    match &tokens[1..] {
                        [Token::Literal(Literal::String(path)), Token::Newline, ..] => {
                            imports.push(Import {
                                path: path.clone(),
                                decl: tokens[0].clone(),
                            });
                        }
                        [Token::Literal(Literal::String(path))] => {
                            imports.push(Import {
                                path: path.clone(),
                                decl: tokens[0].clone(),
                            });
                        }
                        [token, ..] => {
                            return Err(ParserError::UnexpectedToken(token.clone()));
                        }
                        [] => return Err(ParserError::UnexpectedEndOfInput),
                    }

                    index += 3;

                    continue;
                }
                Token::Newline => {
                    // Just increment once at the end of the loop
                }
                _ => return Err(ParserError::UnexpectedToken(token.clone())),
            }
            index += 1;
        }

        Ok((imports, modules, functions, variables))
    }
}
