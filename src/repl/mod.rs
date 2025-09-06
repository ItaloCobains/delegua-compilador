//! # REPL Module
//!
//! Interactive Read-Eval-Print Loop for the DC language.
//! Provides an interactive environment for testing and experimenting
//! with DC code snippets.

use reedline::{DefaultPrompt, Reedline, Signal, FileBackedHistory};
use inkwell::context::Context;

use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
use crate::core::codegen::CodeGen;
use crate::core::error::CompilerError;

/// REPL session manager
pub struct Repl {
    codegen: CodeGen<'static>,
    line_editor: Reedline,
    prompt: DefaultPrompt,
}

impl Repl {
    /// Creates a new REPL session
    pub fn new() -> Result<Self, CompilerError> {
        let context = Box::leak(Box::new(Context::create()));
        let codegen = CodeGen::new(context, "repl")?;

        let mut line_editor = Reedline::create();
        let prompt = DefaultPrompt::default();

        // Initialize persistent history
        if let Ok(history) = FileBackedHistory::with_file(1000, "dc_history.txt".into()) {
            line_editor = line_editor.with_history(Box::new(history));
        }

        Ok(Repl {
            codegen,
            line_editor,
            prompt,
        })
    }

    /// Runs the REPL main loop
    pub fn run(&mut self) -> Result<(), CompilerError> {
        self.print_welcome();

        loop {
            match self.line_editor.read_line(&self.prompt) {
                Ok(Signal::Success(buffer)) => {
                    let input = buffer.trim();

                    if input.is_empty() {
                        continue;
                    }

                    if let Some(command) = self.handle_special_commands(input) {
                        match command {
                            ReplCommand::Exit => break,
                            ReplCommand::Continue => continue,
                        }
                    }

                    match self.evaluate_input(input) {
                        Ok(Some(output)) => {
                            if !output.is_empty() && output != "<sem saída>" {
                                println!("{}", output);
                            }
                        }
                        Ok(None) => {
                            // Statement executed successfully but no output
                        }
                        Err(e) => {
                            println!("Erro: {}", e);
                        }
                    }
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("\nSaindo do REPL...");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Prints the welcome message
    fn print_welcome(&self) {
        println!("=== DC Language REPL ===");
        println!("Digite expressões ou comandos. Use Ctrl+D para sair.");
        println!();
        println!("Exemplos:");
        println!("  var x = 10;");
        println!("  escreva(\"Olá mundo!\");");
        println!("  x + 5");
        println!();
    }

    /// Handles special REPL commands
    fn handle_special_commands(&self, input: &str) -> Option<ReplCommand> {
        match input {
            "quit" | "exit" => Some(ReplCommand::Exit),
            "clear" => {
                println!("Comando 'clear' não implementado ainda.");
                Some(ReplCommand::Continue)
            }
            "help" => {
                self.print_help();
                Some(ReplCommand::Continue)
            }
            "ir" => {
                self.codegen.print_ir();
                Some(ReplCommand::Continue)
            }
            _ => None,
        }
    }

    /// Prints help information
    fn print_help(&self) {
        println!("Comandos disponíveis:");
        println!("  quit/exit - Sair do REPL");
        println!("  clear - Limpar o contexto (não implementado)");
        println!("  help - Mostrar esta ajuda");
        println!("  ir - Mostrar o LLVM IR atual");
        println!();
        println!("Você também pode digitar expressões DC diretamente:");
        println!("  var x = 10;        // Declaração de variável");
        println!("  escreva(\"Olá!\");   // Imprimir texto");
        println!("  x + 5              // Expressões aritméticas");
    }

    /// Evaluates user input
    fn evaluate_input(&mut self, input: &str) -> Result<Option<String>, CompilerError> {
        // Check if it's a simple variable reference
        if Self::is_simple_identifier(input) {
            if self.codegen.has_variable(input) {
                return self.evaluate_variable(input);
            } else {
                return Err(CompilerError::Repl(format!("Variável '{}' não definida", input)));
            }
        }

        // Check if it's a write call
        if Self::is_write_call(input) {
            return self.evaluate_write_call(input);
        }

        // Try to parse as expression first
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(input);
        let mut parser = Parser::new(tokens.clone());

        match parser.parse_expression_only() {
            Ok(expr) => {
                // It's a valid expression - for now just acknowledge it
                Ok(Some(format!("Expressão válida: {:?}", expr)))
            }
            Err(_) => {
                // Try as statement
                let mut parser = Parser::new(tokens);
                let program = parser.parse()?;
                self.codegen.generate(&program)?;
                Ok(None)
            }
        }
    }

    /// Checks if input is a simple identifier
    fn is_simple_identifier(input: &str) -> bool {
        input.chars().all(|c| c.is_alphanumeric() || c == '_') && !input.is_empty()
    }

    /// Checks if input is a write function call
    fn is_write_call(input: &str) -> bool {
        input.starts_with("escreva(") && input.ends_with(")")
    }

    /// Evaluates a variable reference
    fn evaluate_variable(&mut self, name: &str) -> Result<Option<String>, CompilerError> {
        // For now, just check if variable exists
        if self.codegen.has_variable(name) {
            Ok(Some(format!("Variável '{}' existe", name)))
        } else {
            Err(CompilerError::Repl(format!("Variável '{}' não definida", name)))
        }
    }

    /// Evaluates a write function call
    fn evaluate_write_call(&mut self, input: &str) -> Result<Option<String>, CompilerError> {
        let code_with_semicolon = format!("{};", input);
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(&code_with_semicolon);
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        self.codegen.generate(&program)?;
        Ok(Some(Self::extract_write_content(input)))
    }

    /// Extracts content from a write call for display
    fn extract_write_content(input: &str) -> String {
        if let Some(start) = input.find('"') {
            if let Some(end) = input[start+1..].find('"') {
                let content = &input[start+1..start+1+end];
                return format!("\"{}\"", content);
            }
        }
        "executado".to_string()
    }
}

/// Special REPL commands
enum ReplCommand {
    Exit,
    Continue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_simple_identifier() {
        assert!(Repl::is_simple_identifier("x"));
        assert!(Repl::is_simple_identifier("variable_name"));
        assert!(Repl::is_simple_identifier("_private"));
        assert!(!Repl::is_simple_identifier("var x = 10"));
        assert!(!Repl::is_simple_identifier(""));
    }

    #[test]
    fn test_is_write_call() {
        assert!(Repl::is_write_call("escreva(\"hello\")"));
        assert!(!Repl::is_write_call("escreva(\"hello\""));
        assert!(!Repl::is_write_call("print(\"hello\")"));
    }

    #[test]
    fn test_extract_write_content() {
        assert_eq!(Repl::extract_write_content("escreva(\"hello\")"), "\"hello\"");
        assert_eq!(Repl::extract_write_content("escreva(\"a + b\")"), "\"a + b\"");
        assert_eq!(Repl::extract_write_content("escreva(invalid)"), "executado");
    }
}
