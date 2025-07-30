use crate::{
    lex::token::{Literal, Operator, Token},
    parser::modules::Module,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub root: Option<Module>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { root: None }
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    LiteralNull,
    Group {
        inner: Box<Expression>,
    },
    Unary {
        operator: Operator,
        operand: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Operator,
    },
    Variable(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Token,
    pub is_decl: bool,
    pub is_mut: bool,
    pub expression: Box<Expression>,
}
