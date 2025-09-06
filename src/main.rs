mod frontend;
mod backend;

use inkwell::context::Context;
use frontend::lexer::Lexer;
use crate::backend::codegen::CodeGen;
use crate::frontend::parser::Parser;
use reedline::{DefaultPrompt, Reedline, Signal, FileBackedHistory};
use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "repl" => run_repl(),
            "compile" => {
                if args.len() < 3 {
                    eprintln!("Uso: {} compile <arquivo.dc>", args[0]);
                    std::process::exit(1);
                }
                compile_file(&args[2]);
            }
            _ => {
                eprintln!("Uso: {} [repl|compile <arquivo.dc>]", args[0]);
                std::process::exit(1);
            }
        }
    } else {
        // Modo padrão: executar código hardcoded
        run_default();
    }
}

fn run_default() {
    let code = r#"
var a = 10;
var b = 4;
var soma = a + b;
escreva("A soma é: " + texto(soma));
"#;

    let lexer = Lexer::new();
    let tokens = lexer.tokenize(code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "meu_programa").unwrap();
    codegen.generate(&ast).unwrap();

    codegen.print_ir();

    std::fs::write("programa.ll", codegen.get_ir()).unwrap();
}

fn run_repl() {
    println!("=== DC Language REPL ===");
    println!("Digite expressões ou comandos. Use Ctrl+D para sair.");
    println!("Exemplos:");
    println!("  var x = 10;");
    println!("  escreva(\"Olá mundo!\");");
    println!("  x + 5");
    println!();

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "repl").unwrap();

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    // Adicionar histórico persistente
    if let Ok(history) = FileBackedHistory::with_file(1000, "dc_history.txt".into()) {
        line_editor = line_editor.with_history(Box::new(history));
    }

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let input = buffer.trim();

                if input.is_empty() {
                    continue;
                }

                if input == "quit" || input == "exit" {
                    println!("Saindo do REPL...");
                    break;
                }

                if input == "clear" {
                    println!("Comando 'clear' não implementado ainda.");
                    continue;
                }

                if input == "help" {
                    println!("Comandos disponíveis:");
                    println!("  quit/exit - Sair do REPL");
                    println!("  clear - Limpar o contexto");
                    println!("  help - Mostrar esta ajuda");
                    println!("  ir - Mostrar o LLVM IR atual");
                    continue;
                }

                if input == "ir" {
                    codegen.print_ir();
                    continue;
                }

                // Processar a linha diretamente
                match process_repl_input(input, &mut codegen) {
                    Ok(result) => {
                        match result {
                            Some(output) => {
                                if !output.is_empty() && output != "<sem saída>" {
                                    println!("{}", output);
                                }
                            }
                            None => {
                                // Não mostrar "OK" para manter o REPL mais limpo
                            }
                        }
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
}

fn process_repl_input(code: &str, codegen: &mut CodeGen) -> Result<Option<String>, String> {
    let trimmed = code.trim();

    // Verificar se é apenas um identifier (variável)
    if trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') && !trimmed.is_empty() && !trimmed.starts_with("var ") {
        if codegen.has_variable(trimmed) {
            // Para mostrar o valor da variável, vamos criar uma expressão que a referencia
            let expr_code = trimmed.to_string();
            let lexer = Lexer::new();
            let tokens = lexer.tokenize(&expr_code);
            let mut parser = Parser::new(tokens);
            let expr = parser.parse_expression_only()?;
            let ir = codegen.generate_expression_only(&expr)?;
            return Ok(Some(format!("=> {}", evaluate_expression_result(&ir))));
        } else {
            return Err(format!("Erro: Variável '{}' não definida", trimmed));
        }
    }

    // Verificar se é uma chamada de escreva
    if trimmed.starts_with("escreva(") && trimmed.ends_with(")") {
        // É uma chamada de escreva, executar normalmente
        let code_with_semicolon = format!("{};", trimmed);
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(&code_with_semicolon);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        codegen.generate(&ast)?;
        return Ok(Some(format!("=> {}", evaluate_write_expression(trimmed))));
    }

    // Tentar parsear como expressão primeiro
    let lexer = Lexer::new();
    let tokens = lexer.tokenize(trimmed);
    let mut parser = Parser::new(tokens.clone());

    match parser.parse_expression_only() {
        Ok(expr) => {
            // É uma expressão válida, gerar código para ela
            let ir = codegen.generate_expression_only(&expr)?;
            return Ok(Some(format!("=> {}", evaluate_expression_result(&ir))));
        }
        Err(_) => {
            // Não é uma expressão, tentar como statement
            let mut parser = Parser::new(tokens);
            let ast = parser.parse()?;
            codegen.generate(&ast)?;
            
            // Para statements, também precisamos executar o código
            let ir = codegen.get_ir();
            let result = evaluate_expression_result(&ir);
            Ok(Some(result))
        }
    }
}

fn evaluate_write_expression(expr: &str) -> String {
    // Para escreva, vamos extrair o conteúdo da string
    if let Some(start) = expr.find('"') {
        if let Some(end) = expr[start+1..].find('"') {
            let content = &expr[start+1..start+1+end];
            return format!("\"{}\"", content);
        }
    }
    // Se não conseguir extrair, retorna uma mensagem genérica
    "executado".to_string()
}

fn evaluate_expression_result(ir: &str) -> String {
    // Compilar e executar o código para obter o resultado real
    let temp_ir_file = "/tmp/repl_expr.ll";
    if let Err(e) = fs::write(temp_ir_file, ir) {
        return format!("<erro ao salvar IR: {}>", e);
    }

    // Compilar para executável
    let temp_exe = "/tmp/repl_expr";
    let compile_output = Command::new("clang")
        .args(&[temp_ir_file, "-o", temp_exe])
        .output();

    match compile_output {
        Ok(output) if output.status.success() => {
            // Executar e capturar saída
            let exec_output = Command::new(temp_exe)
                .output();

            match exec_output {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    // Limpar arquivos temporários
                    let _ = fs::remove_file(temp_ir_file);
                    let _ = fs::remove_file(temp_exe);

                    if !stdout.trim().is_empty() {
                        stdout.trim().to_string()
                    } else if !stderr.trim().is_empty() {
                        stderr.trim().to_string()
                    } else {
                        "<sem saída>".to_string()
                    }
                }
                Ok(output) => {
                    let _ = fs::remove_file(temp_ir_file);
                    let _ = fs::remove_file(temp_exe);
                    format!("<erro de execução: {}>", String::from_utf8_lossy(&output.stderr))
                }
                Err(e) => {
                    let _ = fs::remove_file(temp_ir_file);
                    let _ = fs::remove_file(temp_exe);
                    format!("<erro ao executar: {}>", e)
                }
            }
        }
        Ok(output) => {
            let _ = fs::remove_file(temp_ir_file);
            format!("<erro de compilação: {}>", String::from_utf8_lossy(&output.stderr))
        }
        Err(e) => {
            let _ = fs::remove_file(temp_ir_file);
            format!("<erro ao compilar: {}>", e)
        }
    }
}

fn compile_file(filename: &str) {
    println!("Compilando {}...", filename);

    // Ler o arquivo
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erro ao ler arquivo {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    // Processar o código
    let lexer = Lexer::new();
    let tokens = lexer.tokenize(&code);
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Erro de parsing: {}", e);
            std::process::exit(1);
        }
    };

    // Gerar LLVM IR
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "programa").unwrap();
    if let Err(e) = codegen.generate(&ast) {
        eprintln!("Erro de geração de código: {}", e);
        std::process::exit(1);
    }

    // Salvar LLVM IR
    let ir_filename = format!("{}.ll", filename.trim_end_matches(".dc"));
    if let Err(e) = fs::write(&ir_filename, codegen.get_ir()) {
        eprintln!("Erro ao salvar LLVM IR: {}", e);
        std::process::exit(1);
    }
    println!("✓ LLVM IR gerado: {}", ir_filename);

    // Gerar código assembly
    let asm_filename = format!("{}.s", filename.trim_end_matches(".dc"));
    if let Err(e) = run_command("llc", &["-o", &asm_filename, &ir_filename]) {
        eprintln!("Erro ao gerar assembly: {}", e);
        std::process::exit(1);
    }
    println!("✓ Código assembly gerado: {}", asm_filename);

    // Compilar e linkar para binário
    let binary_filename = filename.trim_end_matches(".dc");
    if let Err(e) = run_command("clang", &[&asm_filename, "-o", binary_filename]) {
        eprintln!("Erro ao gerar binário: {}", e);
        std::process::exit(1);
    }
    println!("✓ Binário executável gerado: {}", binary_filename);

    println!("\nCompilação concluída com sucesso!");
    println!("Execute ./{} para rodar o programa.", binary_filename);
}

fn run_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| format!("Erro ao executar {}: {}", command, e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Comando {} falhou: {}", command, stderr))
    }
}
