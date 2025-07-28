use crate::{
    lex::token::Token,
    parser::{ParserError, ast::Statement},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub name: Token,
    pub type_: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decl: Token,
    pub name: Token,
    pub parameters: Vec<Arg>,
    pub body: Vec<Statement>,
}

impl TryFrom<&[Token]> for Function {
    type Error = ParserError;

    fn try_from(_tokens: &[Token]) -> Result<Self, Self::Error> {
        todo!()
    }
}
