mod frontend;
mod backend;

use inkwell::context::Context;
use frontend::lexer::Lexer;
use crate::backend::codegen::CodeGen;
use crate::frontend::parser::Parser;

fn main() {
    let code = r#"
var a = 10;
var b = 4;
var soma = a + b;
escreva("A soma é: " + texto(soma));
"#;

    // Processar
    let lexer = Lexer::new();
    let tokens = lexer.tokenize(code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Gerar LLVM IR
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "meu_programa").unwrap();
    codegen.generate(&ast).unwrap();

    // Ver o IR gerado
    codegen.print_ir();

    // Salvar em arquivo
    std::fs::write("programa.ll", codegen.get_ir()).unwrap();
}
