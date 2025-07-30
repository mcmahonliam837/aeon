#[cfg(test)]
mod tests {
    use crate::{
        lex::token::{Keyword, Literal, Operator, Token},
        parser::{
            Parser,
            ast::{Ast, Expression, Statement},
            modules::Module,
            parser_error::ParserError,
        },
    };

    #[test]
    fn test_parse_empty_module() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
        ];

        let Ok(result) = Parser::parse(&tokens) else {
            panic!("Failed to parse empty module");
        };

        assert_eq!(
            result,
            Ast {
                root: Some(Module {
                    decl: Token::Keyword(Keyword::Module),
                    name: "Main".to_string(),
                    imports: vec![],
                    modules: vec![],
                    functions: vec![],
                    variables: vec![],
                })
            }
        );
    }

    #[test]
    fn test_parse_simple_module() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert!(ast.root.is_some());

        let module = ast.root.unwrap();
        assert_eq!(module.name, "Main");
        assert!(module.imports.is_empty());
        assert!(module.modules.is_empty());
        assert!(module.functions.is_empty());
        assert!(module.variables.is_empty());
    }

    #[test]
    fn test_parse_module_with_import() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Import),
            Token::Literal(Literal::String("std/io".to_string())),
            Token::Newline,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();

        assert_eq!(module.imports.len(), 1);
        assert_eq!(module.imports[0].path, "std/io");
    }

    #[test]
    fn test_parse_module_with_function() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        println!("{:?}", result);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();

        assert_eq!(module.functions.len(), 1);

        let function = &module.functions[0];
        assert_eq!(function.name, "main");
        assert!(function.parameters.is_empty());
        assert_eq!(function.return_type.name, "void");
        assert!(function.block.statements.is_empty());
    }

    #[test]
    fn test_parse_function_with_parameters() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("add".to_string()),
            Token::OpenParen,
            Token::Identifier("x".to_string()),
            Token::Identifier("i32".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::Identifier("i32".to_string()),
            Token::CloseParen,
            Token::Identifier("i32".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();
        let function = &module.functions[0];

        assert_eq!(function.parameters.len(), 2);

        assert_eq!(
            function.parameters[0].name,
            Token::Identifier("x".to_string())
        );
        assert_eq!(function.parameters[0].type_info.name, "i32");

        assert_eq!(
            function.parameters[1].name,
            Token::Identifier("y".to_string())
        );
        assert_eq!(function.parameters[1].type_info.name, "i32");

        assert_eq!(function.return_type.name, "i32");
    }

    #[test]
    fn test_parse_variable_declaration() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("42".to_string())),
            Token::Newline,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();

        assert_eq!(module.variables.len(), 1);

        let variable = &module.variables[0];
        assert_eq!(variable.name, "x");
        assert!(variable.is_decl);
        assert!(!variable.is_mut);

        match &*variable.expression {
            Expression::Literal(Literal::Number(n)) => assert_eq!(n, "42"),
            _ => panic!("Expected number literal"),
        }
    }

    #[test]
    fn test_parse_nested_modules() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Module),
            Token::Identifier("Inner".to_string()),
            Token::OpenBrace,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("helper".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();

        assert_eq!(module.modules.len(), 1);

        let inner_module = &module.modules[0];
        assert_eq!(inner_module.name, "Inner");
        assert_eq!(inner_module.functions.len(), 1);
        assert_eq!(inner_module.functions[0].name, "helper");
    }

    #[test]
    fn test_parse_function_with_statements() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("10".to_string())),
            Token::Newline,
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();
        let function = &module.functions[0];

        assert_eq!(function.block.statements.len(), 1);

        match &function.block.statements[0] {
            Statement::Expression(Expression::Variable(var)) => {
                assert_eq!(var.name, "x");
                assert!(var.is_decl);
                match &*var.expression {
                    Expression::Literal(Literal::Number(n)) => assert_eq!(n, "10"),
                    _ => panic!("Expected number literal"),
                }
            }
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_reassignment() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("20".to_string())),
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();
        let function = &module.functions[0];

        match &function.block.statements[0] {
            Statement::Expression(Expression::Variable(var)) => {
                assert!(!var.is_decl); // This is a reassignment
                assert_eq!(var.name, "x");
            }
            _ => panic!("Expected variable expression"),
        }
    }

    #[test]
    fn test_parse_error_missing_module() {
        let tokens = vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
        ];

        let result = Parser::parse(&tokens);
        println!("{:?}", result);
        assert!(result.is_err());

        match result {
            Err(ParserError::ModuleNotFound) => {}
            _ => panic!("Expected ModuleNotFound error"),
        }
    }

    #[test]
    fn test_parse_error_module_without_name() {
        let tokens = vec![Token::Keyword(Keyword::Module), Token::OpenBrace];

        let result = Parser::parse(&tokens);
        assert!(result.is_err());

        match result {
            Err(ParserError::ModuleWithoutName) => {}
            _ => panic!("Expected ModuleWithoutName error"),
        }
    }

    #[test]
    fn test_parse_error_unexpected_end_of_input() {
        let tokens = vec![
            Token::Keyword(Keyword::Module),
            Token::Identifier("Main".to_string()),
            Token::Newline,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            // Missing the rest of the function
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_err());

        match result {
            Err(ParserError::UnexpectedEndOfInput) => {}
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_parse_complex_program() {
        let tokens = vec![
            // Module declaration
            Token::Keyword(Keyword::Module),
            Token::Identifier("Calculator".to_string()),
            Token::Newline,
            // Import
            Token::Keyword(Keyword::Import),
            Token::Literal(Literal::String("std/math".to_string())),
            Token::Newline,
            // Global variable
            Token::Identifier("PI".to_string()),
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("3.14159".to_string())),
            Token::Newline,
            // Nested module
            Token::Keyword(Keyword::Module),
            Token::Identifier("Utils".to_string()),
            Token::OpenBrace,
            Token::Keyword(Keyword::Fn),
            Token::Identifier("square".to_string()),
            Token::OpenParen,
            Token::Identifier("x".to_string()),
            Token::Identifier("f64".to_string()),
            Token::CloseParen,
            Token::Identifier("f64".to_string()),
            Token::OpenBrace,
            Token::CloseBrace,
            Token::CloseBrace,
            // Main function
            Token::Keyword(Keyword::Fn),
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::Identifier("void".to_string()),
            Token::OpenBrace,
            Token::Identifier("result".to_string()),
            Token::Operator(Operator::Assign),
            Token::Operator(Operator::Reassign),
            Token::Literal(Literal::Number("42".to_string())),
            Token::CloseBrace,
        ];

        let result = Parser::parse(&tokens);
        assert!(result.is_ok());

        let ast = result.unwrap();
        let module = ast.root.unwrap();

        // Check main module
        assert_eq!(module.name, "Calculator");
        assert_eq!(module.imports.len(), 1);
        assert_eq!(module.imports[0].path, "std/math");
        assert_eq!(module.variables.len(), 1);
        assert_eq!(module.modules.len(), 1);
        assert_eq!(module.functions.len(), 1);

        // Check global variable
        let global_var = &module.variables[0];
        assert_eq!(global_var.name, "PI");

        // Check nested module
        let utils_module = &module.modules[0];
        assert_eq!(utils_module.name, "Utils");
        assert_eq!(utils_module.functions.len(), 1);
        assert_eq!(utils_module.functions[0].name, "square");

        // Check main function
        let main_fn = &module.functions[0];
        assert_eq!(main_fn.name, "main");
        assert_eq!(main_fn.block.statements.len(), 1);
    }
}
