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

// Helper struct for pretty printing with indentation
pub struct PrettyPrinter {
    indent_str: String,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {
            indent_str: "  ".to_string(),
        }
    }

    pub fn print(&self, ast: &Ast) -> String {
        match &ast.root {
            Some(module) => self.print_module(module, 0),
            None => "<empty AST>".to_string(),
        }
    }

    fn indent(&self, level: usize) -> String {
        self.indent_str.repeat(level)
    }

    fn print_module(&self, module: &Module, level: usize) -> String {
        let mut result = String::new();
        let indent = self.indent(level);

        result.push_str(&format!("{}module {} {{\n", indent, module.name));

        // Print variables
        for var in &module.variables {
            result.push_str(&self.print_variable(var, level + 1));
        }

        // Print functions
        for func in &module.functions {
            if !module.variables.is_empty() && module.functions.first() == Some(func) {
                result.push('\n');
            }
            result.push_str(&self.print_function(func, level + 1));
        }

        // Print nested modules
        for nested in &module.modules {
            if (!module.variables.is_empty() || !module.functions.is_empty())
                && module.modules.first() == Some(nested)
            {
                result.push('\n');
            }
            result.push_str(&self.print_module(nested, level + 1));
        }

        result.push_str(&format!("{}}}\n", indent));
        result
    }

    fn print_variable(&self, var: &Variable, level: usize) -> String {
        let indent = self.indent(level);
        format!("{}{} := {}\n", indent, var.name, var.expression)
    }

    fn print_function(&self, func: &crate::parser::functions::Function, level: usize) -> String {
        let mut result = String::new();
        let indent = self.indent(level);

        result.push_str(&format!("{}fn {}(", indent, func.name));

        // Print parameters
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{} {}", param.name, param.type_));
        }

        result.push_str(&format!(") {} {{\n", func.return_type));

        // Print function body
        for statement in &func.statements {
            result.push_str(&self.print_statement(statement, level + 1));
        }

        result.push_str(&format!("{}}}\n", indent));
        result
    }

    fn print_statement(&self, statement: &Statement, level: usize) -> String {
        let indent = self.indent(level);
        match statement {
            Statement::Variable(var) => {
                format!("{}{} = {}\n", indent, var.name, var.expression)
            }
            Statement::Expression(expr) => {
                format!("{}{}\n", indent, expr)
            }
        }
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}
