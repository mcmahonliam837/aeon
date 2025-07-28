use crate::{
    lex::token::{Operator, Token},
    parser::{ParserContext, ast::Expression, parse_expression, parser_error::ParserError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Token,
    pub expression: Expression,
}

pub fn parse_variable(
    ctx: &mut ParserContext,
    tokens: &[Token],
) -> Result<(Variable, usize), ParserError> {
    println!("Parsing variable from: {:?}", tokens);
    if tokens.len() < 3 {
        return Err(ParserError::UnexpectedEndOfInput);
    }

    if tokens[1] != Token::Operator(Operator::Assign) {
        return Err(ParserError::UnexpectedToken(tokens[1].clone()));
    }

    let name = tokens[0].clone();

    let (expression, token_length) = parse_expression(ctx, &tokens[2..])?;

    Ok((Variable { name, expression }, token_length + 2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex::token::{Keyword, Literal, Operator, Token};
    use crate::parser::ast::Expression;

    #[test]
    fn test_parse_variable_simple() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("42".to_string())),
        ];

        let result = parse_variable(&tokens);
        assert!(result.is_ok());

        let (variable, token_count) = result.unwrap();
        assert_eq!(variable.name, Token::Identifier("x".to_string()));
        match variable.expression {
            Expression::Literal(lit) => {
                assert_eq!(lit, Literal::Number("42".to_string()));
            }
            _ => panic!("Expected literal expression"),
        }
        assert_eq!(token_count, 3);
    }

    #[test]
    fn test_parse_variable_with_trailing_tokens() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("42".to_string())),
            Token::Newline,
            Token::Keyword(Keyword::Module),
        ];

        let result = parse_variable(&tokens);
        assert!(result.is_ok());

        let (variable, token_count) = result.unwrap();
        assert_eq!(variable.name, Token::Identifier("x".to_string()));
        match variable.expression {
            Expression::Literal(lit) => {
                assert_eq!(lit, Literal::Number("42".to_string()));
            }
            _ => panic!("Expected literal expression"),
        }
        // Should only consume the variable assignment tokens, not the trailing ones
        assert_eq!(token_count, 4); // name + = + literal + newline
    }
}
