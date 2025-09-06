//! # Compiler Module
//!
//! Handles file compilation from DC source code to executable binaries.
//! Manages the complete compilation pipeline including LLVM IR generation,
//! assembly code generation, and linking.

use std::fs;
use std::process::Command;
use inkwell::context::Context;

use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
use crate::core::codegen::CodeGen;
use crate::core::error::CompilerError;

/// Compiles a DC source file to an executable
pub fn compile_file(filename: &str) -> Result<(), CompilerError> {
    println!("Compilando {}...", filename);

    // Read source file
    let source_code = fs::read_to_string(filename)
        .map_err(|e| CompilerError::Io(e))?;

    // Parse source code
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(&source_code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // Generate LLVM IR
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "program")?;
    codegen.generate(&ast)?;

    // Save LLVM IR
    let ir_filename = format!("{}.ll", filename.trim_end_matches(".delegua").trim_end_matches(".dc"));
    fs::write(&ir_filename, codegen.get_ir())
        .map_err(|e| CompilerError::Io(e))?;
    println!("✓ LLVM IR gerado: {}", ir_filename);

    // Generate assembly code
    let asm_filename = format!("{}.s", filename.trim_end_matches(".delegua").trim_end_matches(".dc"));
    run_command("llc", &["-o", &asm_filename, &ir_filename])?;
    println!("✓ Código assembly gerado: {}", asm_filename);

    // Compile and link to binary
    let binary_filename = filename.trim_end_matches(".delegua").trim_end_matches(".dc");
    run_command("clang", &[&asm_filename, "-o", binary_filename])?;
    println!("✓ Binário executável gerado: {}", binary_filename);

    println!("\nCompilação concluída com sucesso!");
    println!("Execute ./{} para rodar o programa.", binary_filename);

    Ok(())
}

/// Runs a shell command and returns the result
fn run_command(command: &str, args: &[&str]) -> Result<(), CompilerError> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| CompilerError::CodeGen(format!("Erro ao executar {}: {}", command, e)))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CompilerError::CodeGen(format!("Comando {} falhou: {}", command, stderr)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compile_simple_program() {
        // Create a simple test program
        let test_code = r#"
            var a = 10;
            var b = 5;
            var soma = a + b;
            escreva("Resultado: " + texto(soma));
        "#;

        let test_filename = "test_compile.dc";
        fs::write(test_filename, test_code).unwrap();

        // This test would require LLVM tools to be installed
        // For now, we'll just test the parsing part
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize(test_code);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.statements.len(), 4);

        // Clean up
        let _ = fs::remove_file(test_filename);
    }
}
