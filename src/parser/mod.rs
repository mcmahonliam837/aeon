pub mod ast;
pub mod functions;
pub mod modules;
pub mod parser_error;
pub mod variables;

use crate::{
    lex::token::{Operator, Token},
    parser::{
        ast::{Ast, Expression, Statement},
        modules::parse_module,
        parser_error::ParserError,
        variables::parse_variable,
    },
};

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Ast, ParserError> {
        let module = parse_module(tokens)?;
        Ok(Ast { root: Some(module) })
    }
}

pub fn parse_expression(tokens: &[Token]) -> Result<(Expression, usize), ParserError> {
    println!("Parsing expression: {:?}", tokens);
    match tokens {
        [Token::Literal(literal), Token::Newline, ..] => {
            Ok((Expression::Literal(literal.clone()), 2))
        }
        [Token::Literal(literal)] => Ok((Expression::Literal(literal.clone()), 1)),
        [] => Err(ParserError::UnexpectedEndOfInput),
        _ => Err(ParserError::UnexpectedToken(tokens[0].clone())),
    }
}

pub fn parse_statement(tokens: &[Token]) -> Result<(Statement, usize), ParserError> {
    println!("Parsing statement: {:?}", tokens);
    match tokens {
        [Token::Literal(_), Token::Operator(Operator::Assign), ..] => {
            let (variable, token_length) = parse_variable(tokens)?;
            Ok((Statement::Variable(variable), token_length))
        }
        tokens if !tokens.is_empty() => {
            let (expression, token_length) = parse_expression(tokens)?;
            Ok((Statement::Expression(expression), token_length))
        }
        _ => Err(ParserError::UnexpectedEndOfInput),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        lex::token::{Keyword, Literal, Operator},
        parser::{modules::Module, variables::Variable},
    };

    use super::*;

    #[test]
    fn test_parse_empty_program() {
        let tokens = vec![];
        let ast = Parser::parse(&tokens);
        assert!(matches!(ast.err(), Some(ParserError::ModuleNotFound)));
    }

    #[test]
    fn test_parse_program_with_module() {
        let tokens = vec![Token::Keyword(Keyword::Module)];
        let ast = Parser::parse(&tokens);
        assert!(matches!(ast.err(), Some(ParserError::ModuleWithoutName)));
    }

    #[test]
    fn test_parse_program_with_module_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("main".to_string()),
        ];
        let ast = Parser::parse(&tokens);
        assert_eq!(
            ast.err(),
            Some(ParserError::ModuleEmpty {
                start: Token::Keyword(Keyword::Module),
                end: Token::Identifier("main".to_string())
            })
        );
    }

    #[test]
    fn test_parse_program_with_module_name_and_body() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("main".to_string()),
            Token::Newline,
            Token::Identifier("global_variable".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("1".to_string())),
        ];

        let ast = Parser::parse(&tokens);
        let Ok(ast) = ast else {
            panic!("Expected Ok(Ast), got Err {:?}", ast.err());
        };

        let expected = Ast {
            root: Some(Module {
                name: Token::Identifier("main".to_string()),
                modules: vec![],
                functions: vec![],
                variables: vec![Variable {
                    name: Token::Identifier("global_variable".to_string()),
                    expression: Expression::Literal(Literal::Number("1".to_string())),
                }],
            }),
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_nested_module() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Module),
            Token::Identifier("nested".to_string()),
            Token::OpenBrace,
            Token::Newline,
            Token::Identifier("global_variable".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("1".to_string())),
            Token::Newline,
            Token::CloseBrace,
        ];

        let ast = Parser::parse(&tokens);
        let Ok(ast) = ast else {
            panic!("Expected Ok(Ast), got Err {:?}", ast.err());
        };

        let expected = Ast {
            root: Some(Module {
                name: Token::Identifier("main".to_string()),
                modules: vec![Module {
                    name: Token::Identifier("nested".to_string()),
                    modules: vec![],
                    functions: vec![],
                    variables: vec![Variable {
                        name: Token::Identifier("global_variable".to_string()),
                        expression: Expression::Literal(Literal::Number("1".to_string())),
                    }],
                }],
                functions: vec![],
                variables: vec![],
            }),
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_complex_nested_structure() {
        // module -> module (with variable inside) -> variable -> module(with variable inside) -> variable
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("main".to_string()),
            Token::Newline,
            // First nested module with variable
            Token::Keyword(Keyword::Module),
            Token::Identifier("nested1".to_string()),
            Token::OpenBrace,
            Token::Newline,
            Token::Identifier("var1".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("1".to_string())),
            Token::Newline,
            Token::CloseBrace,
            Token::Newline,
            // Variable at main module level
            Token::Identifier("var2".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("2".to_string())),
            Token::Newline,
            // Second nested module with variable
            Token::Keyword(Keyword::Module),
            Token::Identifier("nested2".to_string()),
            Token::OpenBrace,
            Token::Newline,
            Token::Identifier("var3".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("3".to_string())),
            Token::Newline,
            Token::CloseBrace,
            Token::Newline,
            // Another variable at main module level
            Token::Identifier("var4".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("4".to_string())),
        ];

        let ast = Parser::parse(&tokens);
        let Ok(ast) = ast else {
            panic!("Expected Ok(Ast), got Err {:?}", ast.err());
        };

        let expected = Ast {
            root: Some(Module {
                name: Token::Identifier("main".to_string()),
                modules: vec![
                    Module {
                        name: Token::Identifier("nested1".to_string()),
                        modules: vec![],
                        functions: vec![],
                        variables: vec![Variable {
                            name: Token::Identifier("var1".to_string()),
                            expression: Expression::Literal(Literal::Number("1".to_string())),
                        }],
                    },
                    Module {
                        name: Token::Identifier("nested2".to_string()),
                        modules: vec![],
                        functions: vec![],
                        variables: vec![Variable {
                            name: Token::Identifier("var3".to_string()),
                            expression: Expression::Literal(Literal::Number("3".to_string())),
                        }],
                    },
                ],
                functions: vec![],
                variables: vec![
                    Variable {
                        name: Token::Identifier("var2".to_string()),
                        expression: Expression::Literal(Literal::Number("2".to_string())),
                    },
                    Variable {
                        name: Token::Identifier("var4".to_string()),
                        expression: Expression::Literal(Literal::Number("4".to_string())),
                    },
                ],
            }),
        };

        assert_eq!(ast, expected);
    }
}
