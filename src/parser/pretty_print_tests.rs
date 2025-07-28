#[cfg(test)]
mod tests {
    use crate::lex::token::{Keyword, Literal, Operator, Token};
    use crate::parser::ast::{Ast, Binary, Expression, PrettyPrinter, Statement, Unary};
    use crate::parser::functions::{Arg, Function};
    use crate::parser::modules::Module;
    use crate::parser::variables::Variable;

    #[test]
    fn test_display_literal_expression() {
        let expr = Expression::Literal(Literal::Number("42".to_string()));
        assert_eq!(format!("{}", expr), "42");

        let expr = Expression::Literal(Literal::String("hello".to_string()));
        assert_eq!(format!("{}", expr), "\"hello\"");

        let expr = Expression::Literal(Literal::Boolean(true));
        assert_eq!(format!("{}", expr), "true");
    }

    #[test]
    fn test_display_binary_expression() {
        let left = Box::new(Expression::Literal(Literal::Number("10".to_string())));
        let right = Box::new(Expression::Literal(Literal::Number("20".to_string())));
        let binary = Binary {
            left,
            right,
            operator: Operator::Plus,
        };
        let expr = Expression::Binary(binary);
        assert_eq!(format!("{}", expr), "10 + 20");
    }

    #[test]
    fn test_display_unary_expression() {
        let operand = Box::new(Expression::Literal(Literal::Number("5".to_string())));
        let unary = Unary {
            operator: Operator::Minus,
            operand,
        };
        let expr = Expression::Unary(unary);
        assert_eq!(format!("{}", expr), "-5");
    }

    #[test]
    fn test_display_grouped_expression() {
        let inner = Box::new(Expression::Literal(Literal::Number("42".to_string())));
        let expr = Expression::Group(inner);
        assert_eq!(format!("{}", expr), "(42)");
    }

    #[test]
    fn test_display_variable() {
        let var = Variable {
            name: Token::Identifier("x".to_string()),
            expression: Expression::Literal(Literal::Number("100".to_string())),
        };
        assert_eq!(format!("{}", var), "x := 100");
    }

    #[test]
    fn test_display_statement() {
        let var = Variable {
            name: Token::Identifier("y".to_string()),
            expression: Expression::Literal(Literal::Boolean(false)),
        };
        let stmt = Statement::Variable(var);
        assert_eq!(format!("{}", stmt), "y := false");

        let expr = Expression::Literal(Literal::String("test".to_string()));
        let stmt = Statement::Expression(expr);
        assert_eq!(format!("{}", stmt), "\"test\"");
    }

    #[test]
    fn test_display_empty_ast() {
        let ast = Ast::new();
        assert_eq!(format!("{}", ast), "<empty AST>");
    }

    #[test]
    fn test_pretty_printer_simple_module() {
        let module = Module {
            name: Token::Identifier("main".to_string()),
            modules: vec![],
            functions: vec![],
            variables: vec![Variable {
                name: Token::Identifier("x".to_string()),
                expression: Expression::Literal(Literal::Number("42".to_string())),
            }],
        };

        let ast = Ast { root: Some(module) };

        let printer = PrettyPrinter::new();
        let output = printer.print(&ast);

        let expected = "module main {\n  x := 42\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pretty_printer_module_with_function() {
        let function = Function {
            decl: Token::Keyword(Keyword::Fn),
            name: Token::Identifier("add".to_string()),
            parameters: vec![
                Arg {
                    name: Token::Identifier("a".to_string()),
                    type_: Token::Identifier("int".to_string()),
                },
                Arg {
                    name: Token::Identifier("b".to_string()),
                    type_: Token::Identifier("int".to_string()),
                },
            ],
            return_type: Token::Identifier("int".to_string()),
            statements: vec![Statement::Variable(Variable {
                name: Token::Identifier("result".to_string()),
                expression: Expression::Binary(Binary {
                    left: Box::new(Expression::Literal(Literal::Number("10".to_string()))),
                    right: Box::new(Expression::Literal(Literal::Number("20".to_string()))),
                    operator: Operator::Plus,
                }),
            })],
        };

        let module = Module {
            name: Token::Identifier("math".to_string()),
            modules: vec![],
            functions: vec![function],
            variables: vec![],
        };

        let ast = Ast { root: Some(module) };

        let printer = PrettyPrinter::new();
        let output = printer.print(&ast);

        let expected =
            "module math {\n  fn add(a int, b int) int {\n    result := 10 + 20\n  }\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pretty_printer_nested_modules() {
        let inner_module = Module {
            name: Token::Identifier("inner".to_string()),
            modules: vec![],
            functions: vec![],
            variables: vec![Variable {
                name: Token::Identifier("y".to_string()),
                expression: Expression::Literal(Literal::String("nested".to_string())),
            }],
        };

        let outer_module = Module {
            name: Token::Identifier("outer".to_string()),
            modules: vec![inner_module],
            functions: vec![],
            variables: vec![Variable {
                name: Token::Identifier("x".to_string()),
                expression: Expression::Literal(Literal::Number("1".to_string())),
            }],
        };

        let ast = Ast {
            root: Some(outer_module),
        };

        let printer = PrettyPrinter::new();
        let output = printer.print(&ast);

        let expected =
            "module outer {\n  x := 1\n\n  module inner {\n    y := \"nested\"\n  }\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pretty_printer_complex_expression() {
        // Create expression: (x + 5) * -y
        let x = Expression::Literal(Literal::Number("3".to_string()));
        let five = Expression::Literal(Literal::Number("5".to_string()));
        let add = Expression::Binary(Binary {
            left: Box::new(x),
            right: Box::new(five),
            operator: Operator::Plus,
        });
        let grouped_add = Expression::Group(Box::new(add));

        let y = Expression::Literal(Literal::Number("2".to_string()));
        let neg_y = Expression::Unary(Unary {
            operator: Operator::Minus,
            operand: Box::new(y),
        });

        let multiply = Expression::Binary(Binary {
            left: Box::new(grouped_add),
            right: Box::new(neg_y),
            operator: Operator::Star,
        });

        let var = Variable {
            name: Token::Identifier("result".to_string()),
            expression: multiply,
        };

        let module = Module {
            name: Token::Identifier("test".to_string()),
            modules: vec![],
            functions: vec![],
            variables: vec![var],
        };

        let ast = Ast { root: Some(module) };

        let printer = PrettyPrinter::new();
        let output = printer.print(&ast);

        let expected = "module test {\n  result := (3 + 5) * -2\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_pretty_printer_empty_function() {
        let function = Function {
            decl: Token::Keyword(Keyword::Fn),
            name: Token::Identifier("empty".to_string()),
            parameters: vec![],
            return_type: Token::Identifier("void".to_string()),
            statements: vec![],
        };

        let module = Module {
            name: Token::Identifier("test".to_string()),
            modules: vec![],
            functions: vec![function],
            variables: vec![],
        };

        let ast = Ast { root: Some(module) };

        let printer = PrettyPrinter::new();
        let output = printer.print(&ast);

        let expected = "module test {\n  fn empty() void {\n  }\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_module_display() {
        let module = Module {
            name: Token::Identifier("example".to_string()),
            modules: vec![Module {
                name: Token::Identifier("nested".to_string()),
                modules: vec![],
                functions: vec![],
                variables: vec![],
            }],
            functions: vec![Function {
                decl: Token::Keyword(Keyword::Fn),
                name: Token::Identifier("test".to_string()),
                parameters: vec![],
                return_type: Token::Identifier("void".to_string()),
                statements: vec![],
            }],
            variables: vec![Variable {
                name: Token::Identifier("v".to_string()),
                expression: Expression::Literal(Literal::Boolean(true)),
            }],
        };

        let display_output = format!("{}", module);
        assert!(display_output.contains("module example"));
        assert!(display_output.contains("v := true"));
        assert!(display_output.contains("fn test(...)"));
        assert!(display_output.contains("module nested {...}"));
    }
}
