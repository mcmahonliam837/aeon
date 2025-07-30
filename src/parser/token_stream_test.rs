use crate::lex::token::{Keyword, Literal, Operator, Token};
use crate::parser::parser_error::ParserError;
use crate::parser::token_stream::TokenStream;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tokens() -> Vec<Token> {
        vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::Literal(Literal::Number("42".to_string())),
            Token::CloseBrace,
        ]
    }

    #[test]
    fn test_new_and_basic_properties() {
        let tokens = create_test_tokens();
        let stream = TokenStream::new(&tokens);

        assert_eq!(stream.position(), 0);
        assert!(!stream.is_at_end());
        assert_eq!(stream.remaining().len(), tokens.len());
    }

    #[test]
    fn test_consume_success() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let mut stream = TokenStream::new(&tokens);

        // Consume module keyword
        let result = stream.consume(Token::Keyword(Keyword::Module));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::Keyword(Keyword::Module));
        assert_eq!(stream.position(), 1);

        // Consume identifier (any identifier matches)
        let result = stream.consume(Token::Identifier("anything".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::Identifier("test".to_string()));
        assert_eq!(stream.position(), 2);
        assert!(stream.is_at_end());
    }

    #[test]
    fn test_consume_wrong_token_type() {
        let tokens = vec![Token::Keyword(Keyword::Module)];
        let mut stream = TokenStream::new(&tokens);

        let result = stream.consume(Token::Keyword(Keyword::Fn));
        assert!(matches!(result, Err(ParserError::UnexpectedToken(_))));
        assert_eq!(stream.position(), 0); // Position unchanged on error
    }

    #[test]
    fn test_consume_at_end() {
        let tokens = vec![];
        let mut stream = TokenStream::new(&tokens);

        let result = stream.consume(Token::Keyword(Keyword::Module));
        assert!(matches!(result, Err(ParserError::UnexpectedEndOfInput)));
    }

    #[test]
    fn test_consume_exact() {
        let tokens = vec![
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Plus),
        ];
        let mut stream = TokenStream::new(&tokens);

        // This should succeed - exact match
        let result = stream.consume_exact(Token::Operator(Operator::Assign));
        assert!(result.is_ok());
        assert_eq!(stream.position(), 1);

        // This should fail - wrong operator
        let result = stream.consume_exact(Token::Operator(Operator::Minus));
        assert!(matches!(result, Err(ParserError::UnexpectedToken(_))));
        assert_eq!(stream.position(), 1);
    }

    #[test]
    fn test_advance() {
        let tokens = create_test_tokens();
        let mut stream = TokenStream::new(&tokens);

        // Normal advance
        assert!(stream.advance(3).is_ok());
        assert_eq!(stream.position(), 3);
        assert_eq!(stream.current().unwrap(), &Token::Keyword(Keyword::Fn));

        // Advance to end
        let remaining = tokens.len() - 3;
        assert!(stream.advance(remaining).is_ok());
        assert!(stream.is_at_end());

        // Try to advance past end
        assert!(matches!(
            stream.advance(1),
            Err(ParserError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_current_and_peek() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let mut stream = TokenStream::new(&tokens);

        // Current should return same as peek
        assert_eq!(stream.current().unwrap(), stream.peek().unwrap());
        assert_eq!(stream.current().unwrap(), &Token::Keyword(Keyword::Module));

        // After advance
        stream.advance(1).unwrap();
        assert_eq!(
            stream.current().unwrap(),
            &Token::Identifier("test".to_string())
        );

        // At end
        stream.advance(1).unwrap();
        assert!(stream.current().is_err());
        assert!(stream.peek().is_none());
    }

    #[test]
    fn test_peek_ahead() {
        let tokens = create_test_tokens();
        let stream = TokenStream::new(&tokens);

        assert_eq!(stream.peek_ahead(0), Some(&Token::Keyword(Keyword::Module)));
        assert_eq!(
            stream.peek_ahead(1),
            Some(&Token::Identifier("test".to_string()))
        );
        assert_eq!(stream.peek_ahead(2), Some(&Token::Newline));

        // Past end
        assert_eq!(stream.peek_ahead(100), None);
    }

    #[test]
    fn test_checkpoint_and_restore() {
        let tokens = create_test_tokens();
        let mut stream = TokenStream::new(&tokens);

        // Create checkpoint
        let checkpoint = stream.checkpoint();
        assert_eq!(checkpoint, 0);

        // Advance
        stream.advance(3).unwrap();
        assert_eq!(stream.position(), 3);

        // Create another checkpoint
        let checkpoint2 = stream.checkpoint();
        assert_eq!(checkpoint2, 3);

        // Advance more
        stream.advance(2).unwrap();
        assert_eq!(stream.position(), 5);

        // Restore to second checkpoint
        stream.restore(checkpoint2);
        assert_eq!(stream.position(), 3);

        // Restore to first checkpoint
        stream.restore(checkpoint);
        assert_eq!(stream.position(), 0);
    }

    #[test]
    fn test_try_consume() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let mut stream = TokenStream::new(&tokens);

        // Success case
        let result = stream.try_consume(Token::Keyword(Keyword::Module));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Token::Keyword(Keyword::Module));
        assert_eq!(stream.position(), 1);

        // Failure case - position unchanged
        let result = stream.try_consume(Token::Keyword(Keyword::Fn));
        assert!(result.is_none());
        assert_eq!(stream.position(), 1);

        // At end
        stream.advance(1).unwrap();
        let result = stream.try_consume(Token::Newline);
        assert!(result.is_none());
    }

    #[test]
    fn test_consume_while() {
        let tokens = vec![
            Token::Newline,
            Token::Newline,
            Token::Newline,
            Token::Identifier("test".to_string()),
            Token::Newline,
        ];
        let mut stream = TokenStream::new(&tokens);

        // Consume all newlines
        let consumed = stream.consume_while(|t| matches!(t, Token::Newline));
        assert_eq!(consumed.len(), 3);
        assert_eq!(stream.position(), 3);

        // Current should be identifier
        assert_eq!(
            stream.current().unwrap(),
            &Token::Identifier("test".to_string())
        );

        // Consume nothing (predicate fails immediately)
        let consumed = stream.consume_while(|t| matches!(t, Token::Keyword(_)));
        assert_eq!(consumed.len(), 0);
        assert_eq!(stream.position(), 3);
    }

    #[test]
    fn test_remaining() {
        let tokens = create_test_tokens();
        let mut stream = TokenStream::new(&tokens);

        assert_eq!(stream.remaining().len(), tokens.len());

        stream.advance(3).unwrap();
        assert_eq!(stream.remaining().len(), tokens.len() - 3);
        assert_eq!(stream.remaining()[0], Token::Keyword(Keyword::Fn));

        // Advance to end
        stream.advance(tokens.len() - 3).unwrap();
        assert_eq!(stream.remaining().len(), 0);
    }

    #[test]
    fn test_is_at_end() {
        let tokens = vec![Token::Keyword(Keyword::Module)];
        let mut stream = TokenStream::new(&tokens);

        assert!(!stream.is_at_end());

        stream.advance(1).unwrap();
        assert!(stream.is_at_end());

        // Peek returns None at end
        assert!(stream.peek().is_none());

        // Current returns error at end
        assert!(stream.current().is_err());
    }

    #[test]
    fn test_complex_parsing_scenario() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::Identifier("x".to_string()),
            Token::Identifier("i32".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::Identifier("i32".to_string()),
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];
        let mut stream = TokenStream::new(&tokens);

        // Parse function declaration
        assert!(stream.consume(Token::Keyword(Keyword::Fn)).is_ok());
        let name = stream.consume(Token::Identifier(String::new())).unwrap();
        assert_eq!(name, Token::Identifier("main".to_string()));
        assert!(stream.consume(Token::OpenParen).is_ok());

        // Parse parameters with checkpointing
        let mut params = Vec::new();
        while !matches!(stream.peek(), Some(Token::CloseParen)) {
            let checkpoint = stream.checkpoint();

            // Try to parse parameter
            match stream.consume(Token::Identifier(String::new())) {
                Ok(Token::Identifier(param_name)) => {
                    match stream.consume(Token::Identifier(String::new())) {
                        Ok(Token::Identifier(param_type)) => {
                            params.push((param_name, param_type));
                            // Optional comma
                            stream.try_consume(Token::Comma);
                        }
                        Err(_) => {
                            stream.restore(checkpoint);
                            break;
                        }
                        _ => unreachable!("consume should only return the expected token type"),
                    }
                }
                Err(_) => {
                    stream.restore(checkpoint);
                    break;
                }
                _ => unreachable!("consume should only return the expected token type"),
            }
        }

        assert_eq!(params.len(), 2);
        assert_eq!(params[0], ("x".to_string(), "i32".to_string()));
        assert_eq!(params[1], ("y".to_string(), "i32".to_string()));

        // Continue parsing
        assert!(stream.consume(Token::CloseParen).is_ok());
        assert!(
            stream
                .consume(Token::Identifier("void".to_string()))
                .is_ok()
        );
        assert!(stream.consume(Token::OpenBrace).is_ok());
        assert!(stream.consume(Token::CloseBrace).is_ok());
        assert!(stream.is_at_end());
    }
}
