# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Aeon is a programming language compiler written in Rust, designed as "a better C" that fixes annoying aspects of C without adding fancy new concepts.

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Parse and pretty-print `hello_world.aeon`
- `cargo run --example pretty_print_ast` - Run AST pretty printer demo

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test
- `cargo test --package aeon --lib parser` - Run parser module tests

### Code Quality
- `cargo check` - Quick compile check
- `cargo clippy` - Lint code (if clippy is available)
- `cargo fmt` - Format code (if rustfmt is available)

## Architecture

The codebase is organized into two main modules:

### Lexer (`src/lex/`)
- **lexer.rs** - Main lexing logic that converts source code into tokens
- **token.rs** - Token definitions (Keywords, Literals, Operators, etc.)
- Entry point: `Lexer::lex()` takes a `BufReader` and returns `Vec<Token>`

### Parser (`src/parser/`)
- **parser.rs** - Main parser logic with `ParserContext` for tracking state
- **ast.rs** - AST node definitions and `PrettyPrinter` for formatted output
- **modules.rs** - Module parsing logic
- **functions.rs** - Function parsing with parameters and return types
- **variables.rs** - Variable declaration parsing
- **expression.rs** - Expression parsing (binary, unary, grouping)
- **statement.rs** - Statement parsing logic
- **parser_error.rs** - Parser error definitions
- Entry point: `Parser::parse()` takes tokens and returns `Result<Ast, ParserError>`

### Key Data Structures
- `Ast` - Root AST structure containing optional module
- `Module` - Contains name, nested modules, functions, and variables
- `Function` - Has declaration, name, parameters, return type, and statements
- `Expression` - Supports literals, binary/unary operations, grouping
- `ParserContext` - Tracks current module and function context during parsing

### Main Pipeline
1. `main.rs` reads `hello_world.aeon`
2. `Lexer::lex()` tokenizes the input
3. `Parser::parse()` builds the AST
4. `PrettyPrinter` formats the output

## Sample Code Structure

The Aeon language uses modules as the primary organizational unit:

```aeon
module main

fn main() void {
    str := "Hello, World!"
    i := 0
}
```

## Testing

The parser has comprehensive tests covering:
- Empty programs and error cases
- Module parsing with names and bodies
- Nested module structures
- Complex expression parsing
- Variable declarations

Tests are located in `src/parser/mod.rs` and use the pattern of creating token sequences and verifying the resulting AST structure matches expectations.
