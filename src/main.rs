#![feature(if_let_guard)]

use std::io::BufReader;

use crate::lex::lexer::Lexer;
use crate::parser::Parser;

pub mod lex;
pub mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("hello_world.aeon")?;
    let mut reader = BufReader::new(file);

    let tokens = Lexer::lex(&mut reader)?;

    let _ast = Parser::parse(&tokens)?;

    // ast.iter().for_each(|node| {
    //     println!("{:?}", node);
    // });

    Ok(())
}
