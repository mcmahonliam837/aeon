#![feature(if_let_guard)]

use std::io::BufReader;

use crate::lex::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::pretty_print::PrettyPrinter;

pub mod lex;
pub mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("hello_world.aeon")?;
    let mut reader = BufReader::new(file);

    let tokens = Lexer::lex(&mut reader)?;

    let ast = Parser::parse(&tokens)?;

    let printer = PrettyPrinter::new();
    println!("{}", printer.print(&ast));

    Ok(())
}
