#![feature(if_let_guard)]

use crate::lexer::Lexer;

pub mod lexer;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::lex("hello_world.aeon")?;

    tokens.iter().for_each(|token| {
        println!("{:?}", token);
    });

    Ok(())
}
