use crate::{
    lex::token::{Literal, Operator},
    parser::{modules::Module, variables::Variable},
};
use std::fmt;

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
    Variable(Variable),
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
    pub operator: Operator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.root {
            Some(module) => write!(f, "{}", module),
            None => write!(f, "<empty AST>"),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Variable(var) => write!(f, "{}", var),
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Group(expr) => write!(f, "({})", expr),
            Expression::Unary(unary) => write!(f, "{}", unary),
            Expression::Binary(binary) => write!(f, "{}", binary),
        }
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.operand)
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}
