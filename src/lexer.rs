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
        let mut lexer = Lexer::new(file_path);
        lexer.run()?;
        Ok(lexer.tokens)
    }

    fn new(file_name: &str) -> Self {
        Self {
            file_path: String::from(file_name),
            current_string: String::new(),
            tokens: Vec::new(),
            state: VecDeque::new(),
        }
    }

    fn run(&mut self) -> Result<(), LexerError> {
        let input = std::fs::File::open(self.file_path.clone())?;
        let mut reader = BufReader::new(input);

        let mut position = 0;
        let mut tmp = reader.chars().peekable();
        while let Some(c) = tmp.next() {
            match c {
                Ok(c) => {
                    let peak = tmp.peek().and_then(|r| r.as_ref().ok()).copied();
                    let Some(command) = self.process_byte(c, peak) else {
                        continue;
                    };
                    match command {
                        PostProcessingCommand::Clear => {
                            self.current_string.clear();
                        }
                        PostProcessingCommand::ClearAndSkipPeak => {
                            self.current_string.clear();
                            tmp.next();
                        }
                    }
                }
                Err(ref err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(err) => {
                    return Err(LexerError::IoError(err));
                }
            }
            position += 1;
        }

        Ok(())
    }

    fn process_byte(&mut self, c: char, peak: Option<char>) -> Option<PostProcessingCommand> {
        match self.state.back() {
            None => match c {
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
                    if !self.current_string.is_empty() {
                        let token = Self::process_unknown_string(self.current_string.as_str());
                        self.tokens.push(token);
                    }
                    self.current_string.clear();
                    None
                }
                c if let Ok(token) = Token::try_from(c) => {
                    Self::commit_and_push(&mut self.tokens, token, self.current_string.as_str());
                    Some(PostProcessingCommand::Clear)
                }
                c if c.is_whitespace() && !self.current_string.is_empty() => {
                    if let Ok(keyword) = Keyword::try_from(self.current_string.as_str()) {
                        self.tokens.push(Token::Keyword(keyword));
                    } else {
                        self.tokens
                            .push(Token::Identifier(self.current_string.clone()));
                    }
                    Some(PostProcessingCommand::Clear)
                }
                c if c.is_whitespace() => None,
                c => {
                    let mut operator_chars = vec![c];
                    if let Some(peak) = peak {
                        operator_chars.push(peak);
                    };

                    let op_str = String::from_iter(operator_chars);

                    if let Ok(op) = Operator::try_from(op_str.as_str()) {
                        Self::commit_and_push(
                            &mut self.tokens,
                            Token::Operator(op),
                            self.current_string.as_str(),
                        );
                        return Some(PostProcessingCommand::ClearAndSkipPeak);
                    }

                    if let Ok(op) = Operator::try_from(c.to_string().as_str()) {
                        Self::commit_and_push(
                            &mut self.tokens,
                            Token::Operator(op),
                            self.current_string.as_str(),
                        );

                        return Some(PostProcessingCommand::ClearAndSkipPeak);
                    }

                    self.current_string.push(c);
                    None
                }
            },
            Some(LexerState::InString) => match c {
                '"' if !self.current_string.ends_with("\\") => {
                    self.state.pop_back();
                    self.tokens
                        .push(Token::LiteralString(self.current_string.clone()));
                    Some(PostProcessingCommand::Clear)
                }
                _ => {
                    self.current_string.push(c);
                    None
                }
            },
            Some(LexerState::InComment) => match c {
                '\n' => {
                    self.state.pop_back();
                    None
                }
                _ => None,
            },
        }
    }

    fn commit_and_push(tokens: &mut Vec<Token>, token: Token, str: &str) {
        if !str.is_empty() {
            tokens.push(Self::process_unknown_string(str));
        }
        tokens.push(token);
    }

    fn process_unknown_string(str: &str) -> Token {
        if let Ok(keyword) = Keyword::try_from(str) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(str.to_string())
        }
    }
}
