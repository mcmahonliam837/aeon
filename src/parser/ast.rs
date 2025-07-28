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

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Group(Box<Expression>),
    Unary(Unary),
    Binary(Binary),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    operator: Operator,
    operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Operator,
}
