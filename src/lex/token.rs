#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Literal(Literal),
    OpenParen,
    CloseParen,
    CloseBrace,
    OpenBrace,
    CloseBracket,
    OpenBracket,
    Comma,
    Dot,
    Newline,
}

impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '{' => Ok(Token::OpenBrace),
            '}' => Ok(Token::CloseBrace),
            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Module,
    Import,
    Fn,
    If,
    Else,
    Return,
}

impl TryFrom<&str> for Keyword {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "module" => Ok(Keyword::Module),
            "import" => Ok(Keyword::Import),
            "fn" => Ok(Keyword::Fn),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "return" => Ok(Keyword::Return),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
}

impl TryFrom<&str> for Literal {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            "true" => Ok(Literal::Boolean(true)),
            "false" => Ok(Literal::Boolean(false)),
            s if s.starts_with("0x") => {
                if let Ok(_) = u64::from_str_radix(&s[2..], 16) {
                    Ok(Literal::Number(s.to_string()))
                } else {
                    Err(())
                }
            }
            s if s.starts_with("0b") => {
                if let Ok(_) = u64::from_str_radix(&s[2..], 2) {
                    Ok(Literal::Number(s.to_string()))
                } else {
                    Err(())
                }
            }
            s if s.starts_with("0o") => {
                if let Ok(_) = u64::from_str_radix(&s[2..], 8) {
                    Ok(Literal::Number(s.to_string()))
                } else {
                    Err(())
                }
            }
            s if s.parse::<f64>().is_ok() || s.parse::<u64>().is_ok() => {
                Ok(Literal::Number(s.to_string()))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Assign,
    Reassign,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Less,
    Greater,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Pipeline,
}

impl TryFrom<&str> for Operator {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            ":=" => Ok(Operator::Assign),
            "=" => Ok(Operator::Reassign),
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Star),
            "/" => Ok(Operator::Slash),
            "%" => Ok(Operator::Percent),
            "^" => Ok(Operator::Caret),
            "&" => Ok(Operator::Ampersand),
            "|" => Ok(Operator::Pipe),

            "<" => Ok(Operator::Less),
            ">" => Ok(Operator::Greater),

            "&&" => Ok(Operator::And),
            "||" => Ok(Operator::Or),

            "!=" => Ok(Operator::NotEqual),
            "==" => Ok(Operator::Equal),
            "<=" => Ok(Operator::LessEqual),
            ">=" => Ok(Operator::GreaterEqual),
            "|>" => Ok(Operator::Pipeline),
            _ => Err(()),
        }
    }
}
