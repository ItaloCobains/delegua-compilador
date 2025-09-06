//! # DC Language Core Module
//!
//! This module contains the fundamental data structures and types
//! that define the DC programming language's abstract syntax tree (AST)
//! and token representations.

pub mod ast;
pub mod token;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod codegen;
