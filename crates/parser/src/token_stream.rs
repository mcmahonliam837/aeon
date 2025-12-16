use crate::parser_error::ParserError;
use lex::token::Token;

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> TokenStream<'a> {
    /// Create a new TokenStream from a slice of tokens
    pub fn new(tokens: &'a [Token]) -> Self {
        TokenStream {
            tokens,
            position: 0,
        }
    }

    /// Consume a token if it matches the expected token type
    /// Returns the consumed token on success
    pub fn consume(&mut self, expected: Token) -> Result<Token, ParserError> {
        self.consume_with_message(expected, None)
    }

    /// Consume a token if it matches the expected token type, with a custom error message
    /// Returns the consumed token on success
    pub fn consume_with_message(
        &mut self,
        expected: Token,
        custom_message: Option<&str>,
    ) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        let current = self.current()?;

        let matches = match (&expected, current) {
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Literal(_), Token::Literal(_)) => true,
            _ => current == &expected,
        };

        if matches {
            let token = current.clone();
            self.position += 1;
            Ok(token)
        } else {
            match custom_message {
                Some(_msg) => Err(ParserError::UnexpectedToken(current.clone())),
                None => Err(ParserError::UnexpectedToken(current.clone())),
            }
        }
    }

    /// Consume a specific token exactly (including its value)
    pub fn consume_exact(&mut self, expected: Token) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::UnexpectedEndOfInput);
        }

        let current = self.current()?;

        if *current == expected {
            let token = current.clone();
            self.position += 1;
            Ok(token)
        } else {
            Err(ParserError::UnexpectedToken(current.clone()))
        }
    }

    /// Advance the stream by n tokens
    /// Returns Ok(()) if successful, Err if would go past end
    pub fn advance(&mut self, n: usize) -> Result<(), ParserError> {
        if self.position + n > self.tokens.len() {
            Err(ParserError::UnexpectedEndOfInput)
        } else {
            self.position += n;
            Ok(())
        }
    }

    /// Get the current token without consuming it
    pub fn current(&self) -> Result<&Token, ParserError> {
        self.tokens
            .get(self.position)
            .ok_or(ParserError::UnexpectedEndOfInput)
    }

    pub fn previous(&self) -> Result<&Token, ParserError> {
        if self.position == 0 {
            Err(ParserError::UnexpectedEndOfInput)
        } else {
            self.tokens
                .get(self.position - 1)
                .ok_or(ParserError::UnexpectedEndOfInput)
        }
    }

    pub fn window(&self, n: usize) -> Vec<Option<&Token>> {
        (self.position..self.position + n)
            .map(|range| self.tokens.get(range))
            .collect()
    }

    /// Peek at the current token without consuming it (returns None if at end)
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Peek at the next token without consuming it (returns None if at end)
    pub fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    /// Peek ahead n tokens without consuming (returns None if would go past end)
    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }

    /// Check if we're at the end of the token stream
    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Get the remaining tokens as a slice
    pub fn remaining(&self) -> &'a [Token] {
        &self.tokens[self.position..]
    }

    /// Get the current position in the stream
    pub fn position(&self) -> usize {
        self.position
    }

    /// Create a checkpoint that can be used to restore position later
    pub fn checkpoint(&self) -> usize {
        self.position
    }

    /// Restore to a previous checkpoint
    pub fn restore(&mut self, checkpoint: usize) {
        self.position = checkpoint;
    }

    /// Try to consume a token, returning None if it doesn't match
    pub fn try_consume(&mut self, expected: Token) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        let current = self.peek()?;

        // Check if tokens match based on their type
        let matches = match (&expected, current) {
            // For identifiers and literals, any value matches (structural matching)
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Literal(_), Token::Literal(_)) => true,
            // For keywords, operators, and other tokens, exact match required
            _ => current == &expected,
        };

        if matches {
            let token = current.clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Consume tokens while a predicate is true
    pub fn consume_while<F>(&mut self, mut predicate: F) -> Vec<Token>
    where
        F: FnMut(&Token) -> bool,
    {
        let mut consumed = Vec::new();

        while let Some(token) = self.peek() {
            if predicate(token) {
                consumed.push(token.clone());
                self.position += 1;
            } else {
                break;
            }
        }

        consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lex::token::{Keyword, Token};

    #[test]
    fn test_new_stream() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let stream = TokenStream::new(&tokens);

        assert_eq!(stream.position(), 0);
        assert!(!stream.is_at_end());
    }

    #[test]
    fn test_consume_success() {
        let tokens = vec![Token::Keyword(Keyword::Module)];
        let mut stream = TokenStream::new(&tokens);

        let result = stream.consume(Token::Keyword(Keyword::Module));
        assert!(result.is_ok());
        assert!(stream.is_at_end());
    }

    #[test]
    fn test_consume_wrong_token() {
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
    fn test_advance() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
            Token::Newline,
        ];
        let mut stream = TokenStream::new(&tokens);

        assert!(stream.advance(2).is_ok());
        assert_eq!(stream.position(), 2);

        assert!(stream.advance(2).is_err()); // Would go past end
    }

    #[test]
    fn test_peek_operations() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let stream = TokenStream::new(&tokens);

        assert_eq!(stream.peek(), Some(&Token::Keyword(Keyword::Module)));
        assert_eq!(
            stream.peek_ahead(1),
            Some(&Token::Identifier("test".to_string()))
        );
        assert_eq!(stream.peek_ahead(2), None);
    }

    #[test]
    fn test_checkpoint_restore() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("test".to_string()),
        ];
        let mut stream = TokenStream::new(&tokens);

        let checkpoint = stream.checkpoint();
        stream.advance(1).unwrap();
        assert_eq!(stream.position(), 1);

        stream.restore(checkpoint);
        assert_eq!(stream.position(), 0);
    }
}
