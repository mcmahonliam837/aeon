use crate::token::{Keyword, Operator, Token};
use std::{collections::VecDeque, error::Error, io::BufReader};
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

pub struct Lexer {
    file_path: String,
    current_string: String,
    tokens: Vec<Token>,
    state: VecDeque<LexerState>,
}

impl Lexer {
    pub fn lex(file_path: &str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Self {
            file_path: file_path.to_string(),
            current_string: String::new(),
            tokens: Vec::new(),
            state: VecDeque::new(),
        };
        lexer.run()?;
        Ok(lexer.tokens)
    }

    fn run(&mut self) -> Result<(), LexerError> {
        let input = std::fs::File::open(&self.file_path)?;
        let mut reader = BufReader::new(input);
        let mut chars = reader.chars().peekable();

        while let Some(Ok(c)) = chars.next() {
            let peak = chars.peek().and_then(|r| r.as_ref().ok()).copied();
            let Some(cmd) = self.process_byte(c, peak) else {
                continue;
            };

            self.current_string.clear();
            if matches!(cmd, PostProcessingCommand::ClearAndSkipPeak) {
                chars.next();
            }
        }
        Ok(())
    }

    fn process_byte(&mut self, c: char, peak: Option<char>) -> Option<PostProcessingCommand> {
        match self.state.back() {
            Some(LexerState::InString) => self.process_string(c),
            Some(LexerState::InComment) => {
                if c == '\n' {
                    self.state.pop_back();
                }
                None
            }
            None => self.process_normal(c, peak),
        }
    }

    fn process_string(&mut self, c: char) -> Option<PostProcessingCommand> {
        if c == '"' && !self.current_string.ends_with('\\') {
            self.state.pop_back();
            self.tokens
                .push(Token::LiteralString(self.current_string.clone()));
            Some(PostProcessingCommand::Clear)
        } else {
            self.current_string.push(c);
            None
        }
    }

    fn process_normal(&mut self, c: char, peak: Option<char>) -> Option<PostProcessingCommand> {
        match c {
            '\n' => {
                self.tokens.push(Token::Newline);
                Some(PostProcessingCommand::Clear)
            }
            '"' => {
                self.state.push_back(LexerState::InString);
                None
            }
            '/' if peak == Some('/') => {
                self.state.push_back(LexerState::InComment);
                self.commit_identifier();
                None
            }
            c if c.is_whitespace() => {
                if !self.current_string.is_empty() {
                    self.commit_identifier();
                    Some(PostProcessingCommand::Clear)
                } else {
                    None
                }
            }
            c => self.process_operator_or_char(c, peak),
        }
    }

    fn process_operator_or_char(
        &mut self,
        c: char,
        peak: Option<char>,
    ) -> Option<PostProcessingCommand> {
        if let Ok(token) = Token::try_from(c) {
            self.commit_and_push(token);
            return Some(PostProcessingCommand::Clear);
        }

        let two_char = format!("{}{}", c, peak.unwrap_or(' '));
        if let Ok(op) = Operator::try_from(two_char.as_str()) {
            self.commit_and_push(Token::Operator(op));
            return Some(PostProcessingCommand::ClearAndSkipPeak);
        }

        if let Ok(op) = Operator::try_from(c.to_string().as_str()) {
            self.commit_and_push(Token::Operator(op));
            return Some(PostProcessingCommand::Clear);
        }

        self.current_string.push(c);
        None
    }

    fn commit_and_push(&mut self, token: Token) {
        self.commit_identifier();
        self.tokens.push(token);
    }

    fn commit_identifier(&mut self) {
        if self.current_string.is_empty() {
            return;
        }

        let token = if let Ok(keyword) = Keyword::try_from(self.current_string.as_str()) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(self.current_string.clone())
        };
        self.tokens.push(token);
        self.current_string.clear();
    }
}
