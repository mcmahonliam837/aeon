use crate::lex::token::{Keyword, Literal, Operator, Token};
use std::{collections::VecDeque, error::Error, io::BufRead};
use utf8_chars::BufReadCharsExt;

#[derive(Debug)]
pub enum LexerError {
    IoError(std::io::Error),
    UnexpectedEndOfInput,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::IoError(err) => write!(f, "IO error: {}", err),
            LexerError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
        }
    }
}

impl Error for LexerError {}

impl From<std::io::Error> for LexerError {
    fn from(err: std::io::Error) -> Self {
        LexerError::IoError(err)
    }
}

enum LexerState {
    InString,
    InComment,
}

enum PostProcessingCommand {
    Clear,
    ClearAndSkipPeak,
}

pub struct Lexer<R: BufRead> {
    reader: R,
}

struct LexerContext {
    current_word: String,
    tokens: Vec<Token>,
    state: VecDeque<LexerState>,
}

impl<R: BufRead> Lexer<R> {
    pub fn lex(reader: R) -> Result<Vec<Token>, LexerError> {
        let context = LexerContext {
            current_word: String::new(),
            tokens: Vec::new(),
            state: VecDeque::new(),
        };
        let mut lexer = Self { reader: reader };
        lexer.run(context)
    }

    fn run(&mut self, mut context: LexerContext) -> Result<Vec<Token>, LexerError> {
        let mut chars = self.reader.chars().peekable();

        while let Some(Ok(c)) = chars.next() {
            let peak = chars.peek().and_then(|r| r.as_ref().ok()).copied();
            let Some(cmd) = Self::process_byte(&mut context, c, peak) else {
                continue;
            };

            context.current_word.clear();
            if matches!(cmd, PostProcessingCommand::ClearAndSkipPeak) {
                chars.next();
            }
        }
        // Commit any remaining content at the end of input
        match context.state.back() {
            Some(LexerState::InString) => Err(LexerError::UnexpectedEndOfInput),
            _ => {
                Self::commit_word(&mut context);
                Ok(context.tokens)
            }
        }
    }

    fn process_byte(
        context: &mut LexerContext,
        c: char,
        peak: Option<char>,
    ) -> Option<PostProcessingCommand> {
        match context.state.back() {
            Some(LexerState::InString) => Self::process_string(context, c),
            Some(LexerState::InComment) => {
                if c == '\n' {
                    context.state.pop_back();
                    if Self::should_insert_newline(&context.tokens) {
                        context.tokens.push(Token::Newline);
                    }
                    Some(PostProcessingCommand::Clear)
                } else {
                    None
                }
            }
            None => Self::process_normal(context, c, peak),
        }
    }

    fn process_string(context: &mut LexerContext, c: char) -> Option<PostProcessingCommand> {
        if c == '"' && !context.current_word.ends_with('\\') {
            context.state.pop_back();
            context.tokens.push(Token::Literal(Literal::String(
                context.current_word.clone(),
            )));
            Some(PostProcessingCommand::Clear)
        } else {
            context.current_word.push(c);
            None
        }
    }

    fn process_normal(
        context: &mut LexerContext,
        c: char,
        peak: Option<char>,
    ) -> Option<PostProcessingCommand> {
        match c {
            '\n' => {
                Self::commit_word(context);
                if Self::should_insert_newline(&context.tokens) {
                    context.tokens.push(Token::Newline);
                }
                Some(PostProcessingCommand::Clear)
            }
            '"' => {
                context.state.push_back(LexerState::InString);
                None
            }
            '/' if peak == Some('/') => {
                context.state.push_back(LexerState::InComment);
                Self::commit_word(context);
                None
            }
            c if c.is_whitespace() => {
                if !context.current_word.is_empty() {
                    Self::commit_word(context);
                    Some(PostProcessingCommand::Clear)
                } else {
                    None
                }
            }
            c => Self::process_operator_or_char(context, c, peak),
        }
    }

    fn process_operator_or_char(
        context: &mut LexerContext,
        c: char,
        peak: Option<char>,
    ) -> Option<PostProcessingCommand> {
        if let Ok(token) = Token::try_from(c) {
            Self::commit_and_push(context, token);
            return Some(PostProcessingCommand::Clear);
        }

        let two_char = format!("{}{}", c, peak.unwrap_or(' '));
        if let Ok(op) = Operator::try_from(two_char.as_str()) {
            Self::commit_and_push(context, Token::Operator(op));
            return Some(PostProcessingCommand::ClearAndSkipPeak);
        }

        if let Ok(op) = Operator::try_from(c.to_string().as_str()) {
            Self::commit_and_push(context, Token::Operator(op));
            return Some(PostProcessingCommand::Clear);
        }

        context.current_word.push(c);
        None
    }

    fn commit_and_push(context: &mut LexerContext, token: Token) {
        Self::commit_word(context);
        context.tokens.push(token);
    }

    fn commit_word(context: &mut LexerContext) {
        if context.current_word.is_empty() {
            return;
        }

        let token = if let Ok(keyword) = Keyword::try_from(context.current_word.as_str()) {
            Token::Keyword(keyword)
        } else if let Ok(literal) = Literal::try_from(context.current_word.as_str()) {
            Token::Literal(literal)
        } else {
            Token::Identifier(context.current_word.clone())
        };
        context.tokens.push(token);
        context.current_word.clear();
    }

    fn should_insert_newline(tokens: &[Token]) -> bool {
        tokens.last().map_or(false, |last_token| match last_token {
            Token::CloseBrace => true,
            Token::CloseBracket => true,
            Token::CloseParen => true,
            Token::Identifier(_) => true,
            Token::Literal(_) => true,
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use stringreader::StringReader;

    use super::*;

    fn lex_string(input: &str) -> Result<Vec<Token>, LexerError> {
        let string_reader = StringReader::new(input);
        let reader = BufReader::new(string_reader);
        Lexer::lex(reader)
    }

    #[test]
    fn test_hello_world() {
        let input = "
           module main {
               fn main() {
                   println(\"Hello, world!\")
               }
           }
           ";

        let tokens = lex_string(input).unwrap();

        let expected_tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("main".to_string()),
            Token::OpenBrace,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Identifier("println".to_string()),
            Token::OpenParen,
            Token::Literal(Literal::String("Hello, world!".to_string())),
            Token::CloseParen,
            Token::Newline,
            Token::CloseBrace,
            Token::Newline,
            Token::CloseBrace,
            Token::Newline,
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_empty_input() {
        let tokens = lex_string("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let tokens = lex_string("   \t  \n  \t").unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_keywords() {
        let input = "module import fn if else return";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Keyword(Keyword::Module),
            Token::Keyword(Keyword::Import),
            Token::Keyword(Keyword::Fn),
            Token::Keyword(Keyword::If),
            Token::Keyword(Keyword::Else),
            Token::Keyword(Keyword::Return),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "variable myFunc _private camelCase snake_case CONSTANT";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("variable".to_string()),
            Token::Identifier("myFunc".to_string()),
            Token::Identifier("_private".to_string()),
            Token::Identifier("camelCase".to_string()),
            Token::Identifier("snake_case".to_string()),
            Token::Identifier("CONSTANT".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_single_char_operators() {
        let input = "+ - * / % ^ & | < > =";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Minus),
            Token::Operator(Operator::Star),
            Token::Operator(Operator::Slash),
            Token::Operator(Operator::Percent),
            Token::Operator(Operator::Caret),
            Token::Operator(Operator::Ampersand),
            Token::Operator(Operator::Pipe),
            Token::Operator(Operator::Less),
            Token::Operator(Operator::Greater),
            Token::Operator(Operator::Reassign),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_double_char_operators() {
        let input = ":= == != <= >= && || |>";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Equal),
            Token::Operator(Operator::NotEqual),
            Token::Operator(Operator::LessEqual),
            Token::Operator(Operator::GreaterEqual),
            Token::Operator(Operator::And),
            Token::Operator(Operator::Or),
            Token::Operator(Operator::Pipeline),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_punctuation() {
        let input = "(){}[],.";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::Comma,
            Token::Dot,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_literals() {
        let input = r#""simple" "with spaces" "with\nnewline""#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Literal(Literal::String("simple".to_string())),
            Token::Literal(Literal::String("with spaces".to_string())),
            Token::Literal(Literal::String("with\\nnewline".to_string())),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_escaped_quotes() {
        let input = r#""string with \" escaped quotes""#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Literal(Literal::String(
            r#"string with \" escaped quotes"#.to_string(),
        ))];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_empty_string() {
        let input = r#""""#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Literal(Literal::String("".to_string()))];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let input = "before // this is a comment\nafter";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("before".to_string()),
            Token::Newline,
            Token::Identifier("after".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comment_at_end() {
        let input = "code // comment";
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Identifier("code".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_multiple_comments() {
        let input = "// first comment\ncode\n// second comment\nmore_code";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("code".to_string()),
            Token::Newline,
            Token::Identifier("more_code".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_complex_expression() {
        let input = "x := 5 + 3 * (y - 2)";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("5".to_string())),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Number("3".to_string())),
            Token::Operator(Operator::Star),
            Token::OpenParen,
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Number("2".to_string())),
            Token::CloseParen,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_if_else_statement() {
        let input = "if x > 5 && y < 10 {\n    return true\n} else {\n    return false\n}";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Keyword(Keyword::If),
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Greater),
            Token::Literal(Literal::Number("5".to_string())),
            Token::Operator(Operator::And),
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::Less),
            Token::Literal(Literal::Number("10".to_string())),
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Literal(Literal::Boolean(true)),
            Token::Newline,
            Token::CloseBrace,
            Token::Keyword(Keyword::Else),
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Literal(Literal::Boolean(false)),
            Token::Newline,
            Token::CloseBrace,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_pipeline_operator() {
        let input = "data |> transform |> filter";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("data".to_string()),
            Token::Operator(Operator::Pipeline),
            Token::Identifier("transform".to_string()),
            Token::Operator(Operator::Pipeline),
            Token::Identifier("filter".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_array_access() {
        let input = "arr[0] = arr[i + 1]";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("arr".to_string()),
            Token::OpenBracket,
            Token::Literal(Literal::Number("0".to_string())),
            Token::CloseBracket,
            Token::Operator(Operator::Reassign),
            Token::Identifier("arr".to_string()),
            Token::OpenBracket,
            Token::Identifier("i".to_string()),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Number("1".to_string())),
            Token::CloseBracket,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_method_chaining() {
        let input = "obj.method1().method2().property";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("obj".to_string()),
            Token::Dot,
            Token::Identifier("method1".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Dot,
            Token::Identifier("method2".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Dot,
            Token::Identifier("property".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_adjacent_operators() {
        let input = "x=-5"; // negative number
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Reassign),
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Number("5".to_string())),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_mixed_case_keywords() {
        // Keywords should be case-insensitive according to the Keyword::try_from implementation
        let input = "MODULE Fn RETURN";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Keyword(Keyword::Module),
            Token::Keyword(Keyword::Fn),
            Token::Keyword(Keyword::Return),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_consecutive_newlines() {
        let input = "line1\n\n\nline2";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("line1".to_string()),
            Token::Newline,
            Token::Identifier("line2".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_in_comment() {
        let input = "code // comment with \"string\"";
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Identifier("code".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_division_vs_comment() {
        let input = "a / b // comment";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Slash),
            Token::Identifier("b".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_unclosed_string() {
        // Test that unclosed strings result in an error
        let input = "before \"unclosed string";
        let result = lex_string(input);

        // When a string is not closed, the lexer should return an UnexpectedEndOfInput error
        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_unclosed_string_in_middle() {
        // Test that unclosed strings in the middle of input (with newline) also result in an error
        let input = "before \"unclosed\nafter";
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_closed_string_at_end() {
        // Test that properly closed strings at end of input work correctly
        let input = r#"before "properly closed""#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("before".to_string()),
            Token::Literal(Literal::String("properly closed".to_string())),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_with_multiple_escapes() {
        let input = r#""string with \\ backslash and \" quote""#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Literal(Literal::String(
            r#"string with \\ backslash and \" quote"#.to_string(),
        ))];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators_without_spaces() {
        let input = "a+b-c*d/e%f";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("b".to_string()),
            Token::Operator(Operator::Minus),
            Token::Identifier("c".to_string()),
            Token::Operator(Operator::Star),
            Token::Identifier("d".to_string()),
            Token::Operator(Operator::Slash),
            Token::Identifier("e".to_string()),
            Token::Operator(Operator::Percent),
            Token::Identifier("f".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comparison_chain() {
        let input = "a<b<=c==d!=e>=f>g";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Less),
            Token::Identifier("b".to_string()),
            Token::Operator(Operator::LessEqual),
            Token::Identifier("c".to_string()),
            Token::Operator(Operator::Equal),
            Token::Identifier("d".to_string()),
            Token::Operator(Operator::NotEqual),
            Token::Identifier("e".to_string()),
            Token::Operator(Operator::GreaterEqual),
            Token::Identifier("f".to_string()),
            Token::Operator(Operator::Greater),
            Token::Identifier("g".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_logical_operators() {
        let input = "a && b || c";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::And),
            Token::Identifier("b".to_string()),
            Token::Operator(Operator::Or),
            Token::Identifier("c".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_nested_structures() {
        let input = "{[()]}";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::OpenBrace,
            Token::OpenBracket,
            Token::OpenParen,
            Token::CloseParen,
            Token::CloseBracket,
            Token::CloseBrace,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_mixed_whitespace() {
        let input = "a\t\tb  \t c\n\t d";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Identifier("b".to_string()),
            Token::Identifier("c".to_string()),
            Token::Newline,
            Token::Identifier("d".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_assign_vs_reassign() {
        let input = "x := 5\ny = 10";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Literal(Literal::Number("5".to_string())),
            Token::Newline,
            Token::Identifier("y".to_string()),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("10".to_string())),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comment_with_operators() {
        let input = "code // comment with := != |> operators";
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Identifier("code".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string_followed_by_comment() {
        let input = r#""string" // comment"#;
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Literal(Literal::String("string".to_string()))];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_bitwise_operators() {
        let input = "a & b | c ^ d";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Operator(Operator::Ampersand),
            Token::Identifier("b".to_string()),
            Token::Operator(Operator::Pipe),
            Token::Identifier("c".to_string()),
            Token::Operator(Operator::Caret),
            Token::Identifier("d".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_trailing_whitespace() {
        let input = "identifier   \t  ";
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Identifier("identifier".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_leading_whitespace() {
        let input = "   \t  identifier";
        let tokens = lex_string(input).unwrap();

        let expected = vec![Token::Identifier("identifier".to_string())];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_underscore_identifier() {
        let input = "_ _test __private__";
        let tokens = lex_string(input).unwrap();

        let expected = vec![
            Token::Identifier("_".to_string()),
            Token::Identifier("_test".to_string()),
            Token::Identifier("__private__".to_string()),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_numeric_literals() {
        let input = "123 3.14 0xFF 1e10";
        let tokens = lex_string(input).unwrap();

        // Note: The lexer currently treats all of these as identifiers
        // In a real implementation, these would be parsed as numeric literals
        let expected = vec![
            Token::Literal(Literal::Number("123".to_string())),
            Token::Literal(Literal::Number("3".to_string())),
            Token::Dot,
            Token::Literal(Literal::Number("14".to_string())),
            Token::Literal(Literal::Number("0xFF".to_string())),
            Token::Literal(Literal::Number("1e10".to_string())),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_multiple_unclosed_strings() {
        // Test with properly closed first string and unclosed second string
        let input = r#"first "closed string" second "unclosed"#;
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_string_with_only_quote() {
        // Test input that ends with just a quote (no space before quote)
        let input = "identifier\"";
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_empty_unclosed_string() {
        // Test just a single quote
        let input = r#"""#;
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_newline_in_unclosed_string() {
        // Strings can contain newlines, but still need to be closed
        let input = "\"multiline\nstring\nwithout\nend";
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_escaped_quote_at_end() {
        // Test string ending with escaped quote
        let input = r#""string ending with escaped quote\""#;
        let result = lex_string(input);

        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedEndOfInput) => (),
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }
}
