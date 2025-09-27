# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a compiler for the DC programming language, a simple language with Portuguese keywords built with Rust and LLVM. The language supports variables, arithmetic operations, string operations, and function calls. This compiler implements a Delegua portuguese-like syntax.

Search on GitHub for "Delegua" for more information about the language.

## Common Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Run the REPL
- `cargo run -- compile <filename.delegua>` - Compile a Delegua source file
- `cargo run -- --help` - Show usage information

### Testing
- `cargo test` - Run all tests
- `cargo test unit` - Run unit tests specifically
- `cargo test integration` - Run integration tests specifically
- `cargo test --test unit` - Run unit test suite
- `cargo test --test integration` - Run integration test suite

### Development
- `cargo check` - Check for compilation errors without building
- `cargo clippy` - Run linter for code quality checks
- `cargo fmt` - Format code

## Architecture

### Core Components

The compiler follows a traditional pipeline architecture:

1. **Lexer** (`src/core/lexer.rs`) - Tokenizes DC source code
2. **Parser** (`src/core/parser.rs`) - Builds AST from tokens
3. **CodeGen** (`src/core/codegen.rs`) - Generates LLVM IR from AST

### Key Modules

- `src/core/ast.rs` - Defines the Abstract Syntax Tree structure
- `src/core/token.rs` - Token definitions for the lexer
- `src/core/error.rs` - Error handling types
- `src/cli/mod.rs` - Command-line interface handling
- `src/repl/mod.rs` - Interactive REPL implementation
- `src/compiler/mod.rs` - File compilation logic
- `src/modules/` - Standard library modules (e.g., `matematica.rs`)

### Language Features

The DC language supports:
- Variable declarations with `var` keyword
- Arithmetic expressions (`+`, `-`, `*`, `/`, `%`, `^`)
- Logical operators (`e` for AND, `ou` for OR, `não` for NOT)
- String literals and concatenation
- Function calls like `escreva()` (write) and `texto()` (text conversion)
- Comments with `//`
- Boolean literals (`verdadeiro`, `falso`)

### Dependencies

- **inkwell** - LLVM bindings for code generation (requires LLVM 18.1)
- **reedline** - Enhanced line editing for REPL
- **tempfile** - Temporary file handling for tests
- **criterion** - Benchmarking framework

### Testing Structure

Tests are organized into two main categories:
- **Unit tests** (`tests/unit/mod.rs`) - Test individual components (lexer, parser, codegen)
- **Integration tests** (`tests/integration/mod.rs`) - End-to-end pipeline testing

Both test suites verify the complete compilation pipeline from source code to LLVM IR generation.

### Code Generation

The compiler generates LLVM IR and uses inkwell for LLVM integration. The CodeGen module handles:
- Variable allocation and storage
- Arithmetic operations
- String operations and concatenation
- Function calls (printf for output)
- Type management (integers, strings, functions)

--- 

## Guidelines for Claude Code

When working with this repository, please follow these guidelines:

- Don't write comments in the code unless specifically asked.
- Focus on clarity and maintainability.
- Ensure all new features or changes are covered by tests.
- Follow Rust best practices and idioms.