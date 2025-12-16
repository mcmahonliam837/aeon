pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use insta::assert_debug_snapshot;

    use stringreader::StringReader;

    use super::lexer::*;
    use super::token::*;

    fn lex(path: &str) -> Result<Vec<Token>, LexerError> {
        let source = std::fs::read_to_string(path).expect("failed to load test source");
        let reader = StringReader::new(source.as_str());
        Lexer::lex(BufReader::new(reader))
    }

    #[test]
    fn test_hello_world() {
        assert_debug_snapshot!(lex("../../examples/hello_world.aeon"));
    }

    #[test]
    fn test_modules() {
        assert_debug_snapshot!(lex("../../examples/structs.aeon"));
    }
}
