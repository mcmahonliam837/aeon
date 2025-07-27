#![feature(if_let_guard)]

use std::io::BufReader;

use crate::lex::lexer::Lexer;

pub mod lex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("hello_world.aeon")?;
    let mut reader = BufReader::new(file);

    let tokens = Lexer::lex(&mut reader)?;

    tokens.iter().for_each(|token| {
        println!("{:?}", token);
    });

    Ok(())
}
