use crate::{
    lex::token::{Keyword, Token},
    parser::{ParserContext, ParserError, ast::Statement, parse_statement},
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

pub fn parse_function(
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
    while !matches!(tokens[index], Token::OpenBrace) && !matches!(tokens[index], Token::CloseParen)
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
        let (statement, token_length) = parse_statement(ctx, &tokens[index..])?;
        statements.push(statement);
        index += token_length;
    }

    Ok((statements, index + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::token::{Keyword, Literal, Operator, Token};
    use crate::parser::ast::{Expression, Statement};

    #[test]
    fn test_parse_empty_function() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("test".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        if let Err(err) = result {
            panic!("Unexpected error: {}", err);
        }
        assert!(result.is_ok());

        let (function, token_count) = result.unwrap();
        assert_eq!(function.name, Token::Identifier("test".to_string()));
        assert_eq!(function.parameters.len(), 0);
        assert_eq!(function.statements.len(), 0);
        assert_eq!(function.return_type, Token::Identifier("void".to_string()));
        assert_eq!(token_count, 7);
    }

    #[test]
    fn test_parse_function_with_single_parameter() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("greet".to_string()),
            Token::OpenParen,
            Token::Identifier("name".to_string()),
            Token::Identifier("string".to_string()),
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_ok());

        let (function, token_count) = result.unwrap();
        assert_eq!(function.name, Token::Identifier("greet".to_string()));
        assert_eq!(function.parameters.len(), 1);
        assert_eq!(
            function.parameters[0].name,
            Token::Identifier("name".to_string())
        );
        assert_eq!(
            function.parameters[0].type_,
            Token::Identifier("string".to_string())
        );
        assert_eq!(function.return_type, Token::Identifier("void".to_string()));
        assert_eq!(token_count, 9);
    }

    #[test]
    fn test_parse_function_with_multiple_parameters() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("add".to_string()),
            Token::OpenParen,
            Token::Identifier("a".to_string()),
            Token::Identifier("int".to_string()),
            Token::Comma,
            Token::Identifier("b".to_string()),
            Token::Identifier("int".to_string()),
            Token::CloseParen,
            Token::Identifier("int".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        if let Err(err) = result {
            panic!("Unexpected error: {}", err);
        }
        assert!(result.is_ok());

        let (function, token_count) = result.unwrap();
        assert_eq!(function.name, Token::Identifier("add".to_string()));
        assert_eq!(function.parameters.len(), 2);
        assert_eq!(
            function.parameters[0].name,
            Token::Identifier("a".to_string())
        );
        assert_eq!(
            function.parameters[0].type_,
            Token::Identifier("int".to_string())
        );
        assert_eq!(
            function.parameters[1].name,
            Token::Identifier("b".to_string())
        );
        assert_eq!(
            function.parameters[1].type_,
            Token::Identifier("int".to_string())
        );
        assert_eq!(function.return_type, Token::Identifier("int".to_string()));
        assert_eq!(token_count, 12);
    }

    #[test]
    fn test_parse_function_with_body() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("42".to_string())),
            Token::Newline,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_ok());

        let (function, token_count) = result.unwrap();
        assert_eq!(function.name, Token::Identifier("main".to_string()));
        assert_eq!(function.parameters.len(), 0);
        assert_eq!(function.statements.len(), 1);

        match &function.statements[0] {
            Statement::Variable(var) => {
                assert_eq!(var.name, Token::Identifier("x".to_string()));
                match &var.expression {
                    Expression::Literal(lit) => {
                        assert_eq!(*lit, Literal::Number("42".to_string()));
                    }
                    _ => panic!("Expected literal expression"),
                }
            }
            _ => panic!("Expected variable statement"),
        }

        assert_eq!(function.return_type, Token::Identifier("void".to_string()));
        assert_eq!(token_count, 11);
    }

    #[test]
    fn test_parse_function_missing_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_function_missing_open_paren() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("test".to_string()),
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_function_empty_input() {
        let tokens = vec![];
        let result = parse_function(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_function_wrong_token_type() {
        let tokens = vec![
            Token::Identifier("not_a_function".to_string()),
            Token::Identifier("test".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_function_missing_return_type() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("test".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = parse_function(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_arg_valid() {
        let tokens = vec![
            Token::Identifier("param".to_string()),
            Token::Identifier("string".to_string()),
            Token::Comma,
        ];

        let result = parse_arg(&tokens);
        assert!(result.is_ok());

        let (arg, token_count) = result.unwrap();
        assert_eq!(arg.name, Token::Identifier("param".to_string()));
        assert_eq!(arg.type_, Token::Identifier("string".to_string()));
        assert_eq!(token_count, 2);
    }

    #[test]
    fn test_parse_arg_last_parameter() {
        let tokens = vec![
            Token::Identifier("param".to_string()),
            Token::Identifier("int".to_string()),
            Token::CloseParen,
        ];

        let result = parse_arg(&tokens);
        assert!(result.is_ok());

        let (arg, token_count) = result.unwrap();
        assert_eq!(arg.name, Token::Identifier("param".to_string()));
        assert_eq!(arg.type_, Token::Identifier("int".to_string()));
        assert_eq!(token_count, 2);
    }

    #[test]
    fn test_parse_arg_missing_type() {
        let tokens = vec![Token::Identifier("param".to_string()), Token::Comma];

        let result = parse_arg(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_arg_empty_input() {
        let tokens = vec![];
        let result = parse_arg(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_block_empty() {
        let tokens = vec![Token::OpenBrace, Token::CloseBrace];

        let result = parse_block(&tokens);
        assert!(result.is_ok());

        let (statements, token_count) = result.unwrap();
        assert_eq!(statements.len(), 0);
        assert_eq!(token_count, 2);
    }

    #[test]
    fn test_parse_block_with_statements() {
        let tokens = vec![
            Token::OpenBrace,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("10".to_string())),
            Token::Newline,
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("20".to_string())),
            Token::Newline,
            Token::CloseBrace,
        ];

        let result = parse_block(&tokens);
        assert!(result.is_ok());

        let (statements, token_count) = result.unwrap();
        assert_eq!(statements.len(), 2);
        assert_eq!(token_count, 10);
    }

    #[test]
    fn test_parse_block_missing_open_brace() {
        let tokens = vec![Token::Identifier("x".to_string()), Token::CloseBrace];

        let result = parse_block(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedToken(_))
        ));
    }

    #[test]
    fn test_parse_block_empty_input() {
        let tokens = vec![];
        let result = parse_block(&tokens);
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParserError::UnexpectedEndOfInput)
        ));
    }
}
