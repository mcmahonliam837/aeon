#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    LiteralString(String),
    LiteralInteger(usize),
    LiteralFloat(f64),
    OpenParenthesis,
    CloseParenthesis,
    CloseBrace,
    OpenBrace,
    CloseBracket,
    OpenBracket,
    Comma,
    Dot,
}

impl TryFrom<char> for Token {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            '(' => Ok(Token::OpenParenthesis),
            ')' => Ok(Token::CloseParenthesis),
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
    Integer(usize),
    Float(f64),
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
            _ => Err(()),
        }
    }
}
