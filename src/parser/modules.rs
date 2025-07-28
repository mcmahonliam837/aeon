use crate::{
    lex::token::{Keyword, Token},
    parser::{
        functions::{Function, parse_function},
        parser_error::ParserError,
        variables::{Variable, parse_variable},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Token,
    pub modules: Vec<Module>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

impl TryFrom<&[Token]> for Module {
    type Error = ParserError;

    fn try_from(_tokens: &[Token]) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub fn parse_module(tokens: &[Token]) -> Result<Module, ParserError> {
    validate_module_decl(tokens)?;
    let (modules, functions, variables) = parse_module_body(&tokens[2..])?;
    Ok(Module {
        name: tokens[1].clone(),
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
    validate_module_decl(tokens)?;

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

fn parse_module_body(
    tokens: &[Token],
) -> Result<(Vec<Module>, Vec<Function>, Vec<Variable>), ParserError> {
    let mut modules = Vec::new();
    let mut functions = Vec::new();
    let mut variables = Vec::new();

    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        match token {
            Token::Keyword(Keyword::Module) => {
                let tokens = &tokens[index..];
                validate_nested_module_decl(&tokens[..3])?;
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
                let (inner_modules, inner_functions, inner_variables) =
                    parse_module_body(&tokens[3..body_token_length - 1])?;

                modules.push(Module {
                    name: tokens[1].clone(),
                    modules: inner_modules,
                    functions: inner_functions,
                    variables: inner_variables,
                });

                index += body_token_length + 1;
                continue;
            }
            Token::Identifier(_) => {
                let tokens = &tokens[index..];

                let (variable, token_length) = parse_variable(tokens)?;

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

                let (function, token_length) = parse_function(tokens)?;

                functions.push(function);

                index += token_length + 1;
                continue;
            }
            Token::Newline => {
                // Just increment once at the end of the loop
            }
            _ => return Err(ParserError::UnexpectedToken(token.clone())),
        }
        index += 1;
    }

    Ok((modules, functions, variables))
}
