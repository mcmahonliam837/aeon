use crate::lex::token::{Keyword, Operator, Token};
use std::{collections::VecDeque, error::Error, io::BufRead};
use utf8_chars::BufReadCharsExt;

#[derive(Debug)]
pub enum LexerError {
    IoError(std::io::Error),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::IoError(err) => write!(f, "IO error: {}", err),
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
    current_string: String,
    tokens: Vec<Token>,
    state: VecDeque<LexerState>,
}

impl<R: BufRead> Lexer<R> {
    pub fn lex(reader: R) -> Result<Vec<Token>, LexerError> {
        let context = LexerContext {
            current_string: String::new(),
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

            context.current_string.clear();
            if matches!(cmd, PostProcessingCommand::ClearAndSkipPeak) {
                chars.next();
            }
        }
        Ok(context.tokens)
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
                }
                None
            }
            None => Self::process_normal(context, c, peak),
        }
    }

    fn process_string(context: &mut LexerContext, c: char) -> Option<PostProcessingCommand> {
        if c == '"' && !context.current_string.ends_with('\\') {
            context.state.pop_back();
            context
                .tokens
                .push(Token::LiteralString(context.current_string.clone()));
            Some(PostProcessingCommand::Clear)
        } else {
            context.current_string.push(c);
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
                context.tokens.push(Token::Newline);
                Some(PostProcessingCommand::Clear)
            }
            '"' => {
                context.state.push_back(LexerState::InString);
                None
            }
            '/' if peak == Some('/') => {
                context.state.push_back(LexerState::InComment);
                Self::commit_identifier(context);
                None
            }
            c if c.is_whitespace() => {
                if !context.current_string.is_empty() {
                    Self::commit_identifier(context);
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

        context.current_string.push(c);
        None
    }

    fn commit_and_push(context: &mut LexerContext, token: Token) {
        Self::commit_identifier(context);
        context.tokens.push(token);
    }

    fn commit_identifier(context: &mut LexerContext) {
        if context.current_string.is_empty() {
            return;
        }

        let token = if let Ok(keyword) = Keyword::try_from(context.current_string.as_str()) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(context.current_string.clone())
        };
        context.tokens.push(token);
        context.current_string.clear();
    }
}
