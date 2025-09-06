//! # DC Language Compiler
//!
//! A simple programming language compiler built with Rust and LLVM.
//! Supports basic arithmetic, string operations, and function calls.

mod core;
mod cli;
mod repl;
mod compiler;

use cli::{parse_args, print_usage, print_version, handle_result, CompilerMode};
use repl::Repl;
use compiler::compile_file;

fn main() {
    let args = parse_args();

    match args.mode {
        CompilerMode::Repl => {
            let mut repl = handle_result(Repl::new());
            handle_result(repl.run());
        }
        CompilerMode::Compile(filename) => {
            handle_result(compile_file(&filename));
        }
        CompilerMode::Help => {
            print_usage();
        }
        CompilerMode::Version => {
            print_version();
        }
    }
}
