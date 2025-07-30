use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Keyword(k) => write!(f, "{}", k),
            Token::Operator(op) => write!(f, "{}", op),
            Token::Literal(lit) => write!(f, "{}", lit),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Newline => writeln!(f),
        }
    }
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
    Struct,
    Enum,
    If,
    Else,
    Return,
    Null,
    Void,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Module => write!(f, "module"),
            Keyword::Import => write!(f, "import"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::Return => write!(f, "return"),
            Keyword::Null => write!(f, "null"),
            Keyword::Void => write!(f, "void"),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "module" => Ok(Keyword::Module),
            "import" => Ok(Keyword::Import),
            "fn" => Ok(Keyword::Fn),
            "struct" => Ok(Keyword::Struct),
            "enum" => Ok(Keyword::Enum),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "return" => Ok(Keyword::Return),
            "null" => Ok(Keyword::Null),
            "void" => Ok(Keyword::Void),
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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl TryFrom<&str> for Literal {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            "true" => Ok(Literal::Boolean(true)),
            "false" => Ok(Literal::Boolean(false)),
            s if s.starts_with("0x") => {
                if u64::from_str_radix(&s[2..], 16).is_ok() {
                    Ok(Literal::Number(s.to_string()))
                } else {
                    Err(())
                }
            }
            s if s.starts_with("0b") => {
                if u64::from_str_radix(&s[2..], 2).is_ok() {
                    Ok(Literal::Number(s.to_string()))
                } else {
                    Err(())
                }
            }
            s if s.starts_with("0o") => {
                if u64::from_str_radix(&s[2..], 8).is_ok() {
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
    Question,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Assign => write!(f, ":"),
            Operator::Reassign => write!(f, "="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Star => write!(f, "*"),
            Operator::Slash => write!(f, "/"),
            Operator::Percent => write!(f, "%"),
            Operator::Caret => write!(f, "^"),
            Operator::Ampersand => write!(f, "&"),
            Operator::Pipe => write!(f, "|"),
            Operator::Less => write!(f, "<"),
            Operator::Greater => write!(f, ">"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessEqual => write!(f, "<="),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Pipeline => write!(f, "|>"),
            Operator::Question => write!(f, "?"),
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        match s {
            ":" => Ok(Operator::Assign),
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
            "?" => Ok(Operator::Question),
            _ => Err(()),
        }
    }
}
