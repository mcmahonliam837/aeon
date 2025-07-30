pub mod ast;
pub mod expression;
pub mod functions;
pub mod modules;
pub mod parser_error;
pub mod pretty_print;
pub mod statement;
pub mod variables;

use crate::{
    lex::token::Token,
    parser::{ast::Ast, modules::ModuleParser, parser_error::ParserError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserState {
    current_module: String,
    current_function: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParserContext {
    stack: Vec<ParserState>,
}

impl ParserContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter_module(&mut self, module_name: String) {
        self.stack.push(ParserState {
            current_module: module_name,
            current_function: None,
        });
    }

    pub fn exit_module(&mut self) {
        self.stack.pop();
    }

    pub fn enter_function(&mut self, function_name: String) {
        self.stack.push(ParserState {
            current_module: self
                .stack
                .last()
                .expect("Cannot enter function without module")
                .current_module
                .clone(),
            current_function: Some(function_name),
        });
    }

    pub fn exit_function(&mut self) {
        self.stack.pop();
    }
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Ast, ParserError> {
        let mut ctx = ParserContext::new();
        let module = ModuleParser::parse(&mut ctx, tokens)?;
        Ok(Ast { root: Some(module) })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::BufReader;

    use insta::assert_debug_snapshot;

    use crate::lex::lexer::Lexer;
    use stringreader::StringReader;

    fn parse(path: &str) -> Result<Ast, Box<dyn std::error::Error>> {
        let source = std::fs::read_to_string(path).expect("failed to load test source");
        let reader = StringReader::new(source.as_str());
        let tokens = Lexer::lex(BufReader::new(reader))?;
        Parser::parse(&tokens).map_err(Box::from)
    }

    #[test]
    fn test_hello_world() {
        assert_debug_snapshot!(parse("aeon_examples/hello_world.aeon"));
    }

    #[test]
    fn test_modules() {
        assert_debug_snapshot!(parse("aeon_examples/modules.aeon"));
    }
}
