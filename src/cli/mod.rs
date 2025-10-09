use std::env;
use std::process;
use crate::core::error::CompilerError;

#[derive(Debug)]
pub enum CompilerMode {
    Compile(String),
    Help,
    Version,
}

#[derive(Debug)]
pub struct CliArgs {
    pub mode: CompilerMode,
}

pub fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            CliArgs { mode: CompilerMode::Help }
        }
        2 => {
            match args[1].as_str() {
                "help" | "-h" | "--help" | "ajuda" | "--ajuda" => CliArgs { mode: CompilerMode::Help },
                "version" | "-v" | "--version" | "versao" | "--versao" => CliArgs { mode: CompilerMode::Version },
                _ => {
                    eprintln!("Comando desconhecido: {}", args[1]);
                    print_usage();
                    process::exit(1);
                }
            }
        }
        3 => {
            if args[1] == "construa" {
                CliArgs { mode: CompilerMode::Compile(args[2].clone()) }
            } else {
                eprintln!("Comando desconhecido: {}", args[1]);
                print_usage();
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Muitos argumentos fornecidos.");
            print_usage();
            process::exit(1);
        }
    }
}

pub fn print_usage() {
    println!("Compilador Delegua");
    println!();
    println!("Uso:");
    println!("    delegua-compiler [COMMAND]");
    println!();
    println!("Comandos:");
    println!("    construa <file.delegua>       Compila o arquivo para executável");
    println!("    ajuda                         Mostra esta mensagem de ajuda");
    println!("    versao                      Mostra a versão do compilador");
    println!();
    println!("EXAMPLES:");
    println!("    delegua-compiler construa program.delegua");
}

pub fn print_version() {
    println!("Compilador Delegua v{}", env!("CARGO_PKG_VERSION"));
}

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
    fn test_parse_args_compile() {
        let args = CliArgs { mode: CompilerMode::Compile("test.delegua".to_string()) };
        assert!(matches!(args.mode, CompilerMode::Compile(_)));
    }
}
