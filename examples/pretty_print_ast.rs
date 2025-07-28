use aeon::lex::token::{Keyword, Literal, Operator, Token};
use aeon::parser::ast::{Ast, Binary, Expression, PrettyPrinter, Statement, Unary};
use aeon::parser::functions::{Arg, Function};
use aeon::parser::modules::Module;
use aeon::parser::variables::Variable;

fn main() {
    println!("=== AST Pretty Printer Demo ===\n");

    // Example 1: Simple module with variables
    println!("Example 1: Simple module with variables");
    println!("---------------------------------------");
    let simple_module = create_simple_module();
    let ast1 = Ast {
        root: Some(simple_module),
    };
    print_ast(&ast1);

    // Example 2: Module with functions
    println!("\nExample 2: Module with functions");
    println!("--------------------------------");
    let module_with_functions = create_module_with_functions();
    let ast2 = Ast {
        root: Some(module_with_functions),
    };
    print_ast(&ast2);

    // Example 3: Nested modules
    println!("\nExample 3: Nested modules");
    println!("-------------------------");
    let nested_modules = create_nested_modules();
    let ast3 = Ast {
        root: Some(nested_modules),
    };
    print_ast(&ast3);

    // Example 4: Complex expressions
    println!("\nExample 4: Complex expressions");
    println!("------------------------------");
    let complex_module = create_module_with_complex_expressions();
    let ast4 = Ast {
        root: Some(complex_module),
    };
    print_ast(&ast4);
}

fn print_ast(ast: &Ast) {
    // Using Display trait
    println!("Display trait output:");
    println!("{}", ast);

    // Using PrettyPrinter
    println!("\nPrettyPrinter output:");
    let printer = PrettyPrinter::new();
    println!("{}", printer.print(ast));
}

fn create_simple_module() -> Module {
    Module {
        name: Token::Identifier("config".to_string()),
        modules: vec![],
        functions: vec![],
        variables: vec![
            Variable {
                name: Token::Identifier("version".to_string()),
                expression: Expression::Literal(Literal::String("1.0.0".to_string())),
            },
            Variable {
                name: Token::Identifier("debug".to_string()),
                expression: Expression::Literal(Literal::Boolean(true)),
            },
            Variable {
                name: Token::Identifier("max_connections".to_string()),
                expression: Expression::Literal(Literal::Number("100".to_string())),
            },
        ],
    }
}

fn create_module_with_functions() -> Module {
    Module {
        name: Token::Identifier("math".to_string()),
        modules: vec![],
        functions: vec![
            Function {
                decl: Token::Keyword(Keyword::Fn),
                name: Token::Identifier("add".to_string()),
                parameters: vec![
                    Arg {
                        name: Token::Identifier("x".to_string()),
                        type_: Token::Identifier("int".to_string()),
                    },
                    Arg {
                        name: Token::Identifier("y".to_string()),
                        type_: Token::Identifier("int".to_string()),
                    },
                ],
                return_type: Token::Identifier("int".to_string()),
                statements: vec![Statement::Expression(Expression::Binary(Binary {
                    left: Box::new(Expression::Literal(Literal::Number("x".to_string()))),
                    right: Box::new(Expression::Literal(Literal::Number("y".to_string()))),
                    operator: Operator::Plus,
                }))],
            },
            Function {
                decl: Token::Keyword(Keyword::Fn),
                name: Token::Identifier("factorial".to_string()),
                parameters: vec![Arg {
                    name: Token::Identifier("n".to_string()),
                    type_: Token::Identifier("int".to_string()),
                }],
                return_type: Token::Identifier("int".to_string()),
                statements: vec![
                    Statement::Variable(Variable {
                        name: Token::Identifier("result".to_string()),
                        expression: Expression::Literal(Literal::Number("1".to_string())),
                    }),
                    Statement::Expression(Expression::Literal(Literal::Number(
                        "result".to_string(),
                    ))),
                ],
            },
        ],
        variables: vec![Variable {
            name: Token::Identifier("PI".to_string()),
            expression: Expression::Literal(Literal::Number("3.14159".to_string())),
        }],
    }
}

fn create_nested_modules() -> Module {
    Module {
        name: Token::Identifier("app".to_string()),
        modules: vec![
            Module {
                name: Token::Identifier("database".to_string()),
                modules: vec![],
                functions: vec![],
                variables: vec![
                    Variable {
                        name: Token::Identifier("host".to_string()),
                        expression: Expression::Literal(Literal::String("localhost".to_string())),
                    },
                    Variable {
                        name: Token::Identifier("port".to_string()),
                        expression: Expression::Literal(Literal::Number("5432".to_string())),
                    },
                ],
            },
            Module {
                name: Token::Identifier("server".to_string()),
                modules: vec![],
                functions: vec![Function {
                    decl: Token::Keyword(Keyword::Fn),
                    name: Token::Identifier("start".to_string()),
                    parameters: vec![],
                    return_type: Token::Identifier("void".to_string()),
                    statements: vec![],
                }],
                variables: vec![Variable {
                    name: Token::Identifier("port".to_string()),
                    expression: Expression::Literal(Literal::Number("8080".to_string())),
                }],
            },
        ],
        functions: vec![],
        variables: vec![Variable {
            name: Token::Identifier("name".to_string()),
            expression: Expression::Literal(Literal::String("MyApp".to_string())),
        }],
    }
}

fn create_module_with_complex_expressions() -> Module {
    // Create expression: (10 + 20) * -5
    let add = Expression::Binary(Binary {
        left: Box::new(Expression::Literal(Literal::Number("10".to_string()))),
        right: Box::new(Expression::Literal(Literal::Number("20".to_string()))),
        operator: Operator::Plus,
    });
    let grouped_add = Expression::Group(Box::new(add));

    let neg_five = Expression::Unary(Unary {
        operator: Operator::Minus,
        operand: Box::new(Expression::Literal(Literal::Number("5".to_string()))),
    });

    let multiply = Expression::Binary(Binary {
        left: Box::new(grouped_add),
        right: Box::new(neg_five),
        operator: Operator::Star,
    });

    // Create expression: x < 10 && y > 20
    let x_less_10 = Expression::Binary(Binary {
        left: Box::new(Expression::Literal(Literal::Number("x".to_string()))),
        right: Box::new(Expression::Literal(Literal::Number("10".to_string()))),
        operator: Operator::Less,
    });

    let y_greater_20 = Expression::Binary(Binary {
        left: Box::new(Expression::Literal(Literal::Number("y".to_string()))),
        right: Box::new(Expression::Literal(Literal::Number("20".to_string()))),
        operator: Operator::Greater,
    });

    let and_expr = Expression::Binary(Binary {
        left: Box::new(x_less_10),
        right: Box::new(y_greater_20),
        operator: Operator::And,
    });

    Module {
        name: Token::Identifier("expressions".to_string()),
        modules: vec![],
        functions: vec![],
        variables: vec![
            Variable {
                name: Token::Identifier("complex_math".to_string()),
                expression: multiply,
            },
            Variable {
                name: Token::Identifier("condition".to_string()),
                expression: and_expr,
            },
        ],
    }
}
