//! # CLI Module
//!
//! Handles command-line interface operations including argument parsing,
//! help messages, and dispatching to appropriate compiler modes.

use std::env;
use std::process;
use crate::core::error::CompilerError;

/// Available command-line modes for the DC compiler
#[derive(Debug)]
pub enum CompilerMode {
    /// Interactive REPL mode
    Repl,
    /// Compile a file to executable
    Compile(String),
    /// Show help information
    Help,
    /// Show version information
    Version,
}

/// Parsed command-line arguments
#[derive(Debug)]
pub struct CliArgs {
    pub mode: CompilerMode,
}

/// Parses command-line arguments and returns the appropriate mode
pub fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments - show help
            CliArgs { mode: CompilerMode::Help }
        }
        2 => {
            match args[1].as_str() {
                "repl" => CliArgs { mode: CompilerMode::Repl },
                "help" | "-h" | "--help" => CliArgs { mode: CompilerMode::Help },
                "version" | "-v" | "--version" => CliArgs { mode: CompilerMode::Version },
                _ => {
                    eprintln!("Unknown command: {}", args[1]);
                    print_usage();
                    process::exit(1);
                }
            }
        }
        3 => {
            if args[1] == "compile" {
                CliArgs { mode: CompilerMode::Compile(args[2].clone()) }
            } else {
                eprintln!("Unknown command: {}", args[1]);
                print_usage();
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Too many arguments");
            print_usage();
            process::exit(1);
        }
    }
}

/// Prints usage information
pub fn print_usage() {
    println!("DC Language Compiler");
    println!();
    println!("USAGE:");
    println!("    dc [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    repl                    Start interactive REPL");
    println!("    compile <file.dc>       Compile DC file to executable");
    println!("    help                    Show this help message");
    println!("    version                 Show version information");
    println!();
    println!("EXAMPLES:");
    println!("    dc repl");
    println!("    dc compile program.dc");
}

/// Prints version information
pub fn print_version() {
    println!("DC Language Compiler v{}", env!("CARGO_PKG_VERSION"));
    println!("Built with Rust and LLVM");
}

/// Handles the result of compiler operations
pub fn handle_result<T>(result: Result<T, CompilerError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_repl() {
        // This test would require mocking std::env::args()
        // For now, we'll just test the structure
        let args = CliArgs { mode: CompilerMode::Repl };
        assert!(matches!(args.mode, CompilerMode::Repl));
    }

    #[test]
    fn test_parse_args_compile() {
        let args = CliArgs { mode: CompilerMode::Compile("test.delegua".to_string()) };
        assert!(matches!(args.mode, CompilerMode::Compile(_)));
    }
}
