//! # DC Language Compiler
//!
//! A compiler for the DC programming language, featuring:
//! - Modular architecture with separate concerns
//! - LLVM-based code generation
//! - Interactive REPL
//! - Command-line interface

pub mod core;
pub mod cli;
pub mod repl;
pub mod compiler;
