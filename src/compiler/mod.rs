use std::fs;
use std::process::Command;
use inkwell::context::Context;

use delegua::lexador::Lexador;
use delegua::analise_sintatica::AvaliadorSintatico;
use delegua::gerador_de_codigo::GeradorDeCodigo;
use delegua::error::CompilerError;

pub fn compile_file(filename: &str) -> Result<(), CompilerError> {
    println!("Compilando {}...", filename);

    let source_code = fs::read_to_string(filename)
        .map_err(|e| CompilerError::Io(e.to_string()))?;

    let mut lexer = Lexador::new();
    let tokens = lexer.analisar(&source_code);
    let mut parser = AvaliadorSintatico::new(tokens);
    let ast = parser.analisar()?;

    let context = Context::create();
    let mut codegen = GeradorDeCodigo::new(&context, "program")?;
    codegen.gerar(&ast)?;

    let ir_filename = format!("{}.ll", filename.trim_end_matches(".delegua"));
    fs::write(&ir_filename, codegen.get_ir())
        .map_err(|e| CompilerError::Io(e.to_string()))?;
    println!("✓ LLVM IR gerado: {}", ir_filename);

    let asm_filename = format!("{}.s", filename.trim_end_matches(".delegua"));
    run_command("llc", &["-o", &asm_filename, &ir_filename])?;
    println!("✓ Código assembly gerado: {}", asm_filename);

    let binary_filename = filename.trim_end_matches(".delegua");
    run_command("clang", &[&asm_filename, "-o", binary_filename])?;
    println!("✓ Binário executável gerado: {}", binary_filename);

    println!("\nCompilação concluída com sucesso!");
    println!("Execute ./{} para rodar o programa.", binary_filename);

    Ok(())
}

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

        let test_filename = "test_compile.delegua_compilador";
        fs::write(test_filename, test_code).unwrap();

        let mut lexer = Lexador::new();
        let tokens = lexer.analisar(test_code);
        let mut parser = AvaliadorSintatico::new(tokens);
        let ast = parser.analisar().unwrap();

        assert_eq!(ast.declaracoes.len(), 4);

        let _ = fs::remove_file(test_filename);
    }
}
