use std::fs;

fn main() {
    let source = fs::read_to_string("test_function.dc").expect("Failed to read file");
    
    // Manually create lexer to see what tokens it generates
    println!("Source: {}", source);
    println!("Source bytes: {:?}", source.as_bytes());
    
    let mut lexer = dc::core::lexer::Lexer::new();
    let tokens = lexer.tokenize(&source);
    
    for token in &tokens {
        println!("{:?}", token);
    }
}