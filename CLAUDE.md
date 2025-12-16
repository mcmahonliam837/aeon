# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Aeon is a programming language compiler written in Rust, designed as "a better C" that fixes annoying aspects of C without adding fancy new concepts.

## Development Commands

### Building and Running
- `cargo build` - Build the entire workspace
- `cargo run` - Run the main binary (parses `hello_world.aeon`)
- `cargo build --package lex` - Build only the lexer crate
- `cargo build --package parser` - Build only the parser crate

### Testing
- `cargo test` - Run all tests across the workspace
- `cargo test <test_name>` - Run specific test by name
- `cargo test --package lex` - Run tests for the lexer crate only
- `cargo test --package parser` - Run tests for the parser crate only
- Tests use `insta` for snapshot testing (see `examples/*.aeon` test cases in `parser/src/lib.rs`)

### Code Quality
- `cargo check` - Quick compile check
- `cargo clippy` - Lint code (if clippy is available)
- `cargo fmt` - Format code (if rustfmt is available)

## Architecture

The codebase is organized as a Cargo workspace with separate crates:

### Workspace Structure
- **Root package (`aeon`)** - Main binary that orchestrates lexing and parsing
- **`crates/lex`** - Lexer crate
- **`crates/parser`** - Parser crate

### Lexer (`crates/lex/src/`)
- **lexer.rs** - Main lexing logic that converts source code into tokens
- **token.rs** - Token definitions (Keywords, Literals, Operators, etc.)
- Entry point: `Lexer::lex()` takes a `BufReader` and returns `Vec<Token>`

### Parser (`crates/parser/src/`)
- **lib.rs** - Main parser entry point with `Parser::parse()` and `ParserContext` for tracking state
- **token_stream.rs** - Token stream abstraction with lookahead, checkpointing, and consume operations
- **ast.rs** - AST node definitions and `PrettyPrinter` for formatted output
- **modules.rs** - Module parsing logic
- **functions.rs** - Function parsing with parameters and return types
- **expression.rs** - Expression parsing with precedence-based operator handling
- **statement.rs** - Statement parsing logic
- **block.rs** - Block parsing logic
- **parser_error.rs** - Parser error definitions
- Entry point: `Parser::parse()` takes tokens and returns `Result<Ast, ParserError>`

### Key Data Structures
- `Ast` - Root AST structure containing optional module
- `Module` - Contains name, nested modules, functions, and variables
- `Function` - Has declaration, name, parameters, return type, and statements
- `Expression` - Supports literals, binary/unary operations, grouping with precedence
- `ParserContext` - Stack-based context tracker for nested modules and functions, generates fully qualified names
- `TokenStream` - Wrapper around token slice providing peek, consume, checkpoint/restore operations

### Main Pipeline
1. `main.rs` reads a `.aeon` file (default: `hello_world.aeon`)
2. `Lexer::lex()` tokenizes the input
3. `Parser::parse()` builds the AST using `TokenStream` for token navigation
4. AST is returned (pretty printing functionality available via `PrettyPrinter`)

## Sample Code Structure

The Aeon language supports imports and functions:

```aeon
import "std/io"

fn main() {
    tmp := 1
    // io.printf("Hello, World!\n")
}
```

See `examples/` directory for more sample code including `hello_world.aeon`, `math.aeon`, and `structs.aeon`.

## Testing

The codebase uses `insta` for snapshot testing:

### Parser Tests (`crates/parser/src/`)
- **lib.rs** - Integration tests that parse example `.aeon` files and snapshot the AST
- **parser_test.rs** - Unit tests for parser components
- **token_stream_test.rs** - Unit tests for `TokenStream` functionality

### Test Pattern
Tests typically create token sequences manually or parse `.aeon` files, then verify the resulting AST structure using `assert_debug_snapshot!` from the `insta` crate.
