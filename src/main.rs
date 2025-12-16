use std::io::BufReader;

use lex::lexer::Lexer;
use parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("hello_world.aeon")?;
    let mut reader = BufReader::new(file);

    let tokens = Lexer::lex(&mut reader)?;

    let _ast = Parser::parse(&tokens)?;

    Ok(())
}
