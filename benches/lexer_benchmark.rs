use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dc::core::lexer::Lexer;

fn bench_lexer_tokenization(c: &mut Criterion) {
    let input = black_box("var x = 42; escreva(\"Hello World\"); se (x > 0) { x = x - 1; }");

    c.bench_function("lexer_tokenize", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new();
            let _tokens = lexer.tokenize(input);
        })
    });
}

fn bench_lexer_large_input(c: &mut Criterion) {
    let large_input = black_box(
        "var x = 42; var y = 24; var z = x + y; escreva(\"Result: \"); escreva(texto(z)); \
         se (z > 50) { escreva(\"Greater than 50\"); } senao { escreva(\"Less or equal 50\"); } \
         para (var i = 0; i < 10; i = i + 1) { escreva(texto(i)); } \
         funcao soma(a, b) { retorna a + b; } var result = soma(x, y);"
    );

    c.bench_function("lexer_tokenize_large", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new();
            let _tokens = lexer.tokenize(large_input);
        })
    });
}

criterion_group!(benches, bench_lexer_tokenization, bench_lexer_large_input);
criterion_main!(benches);
