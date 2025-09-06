As a Rust compiler developer, please help optimize the lexer.rs implementation for the Delegua compiler with these requirements:

1. Performance Optimization Requirements:
- Implement zero-copy string handling using string slices where possible
- Use pre-allocated buffers for token storage
- Minimize heap allocations during lexical analysis
- Leverage Rust's efficient iterator patterns
- Consider using lookup tables for common token patterns

2. Memory Usage Requirements:
- Implement memory pooling for frequently allocated token types
- Use appropriate sized integer types for token positions/lengths
- Implement efficient token stream buffering
- Carefully manage string interning
- Consider arena allocation for token lifetimes

3. Best Practices Integration:
- Follow Rust 2021 idioms and patterns
- Implement error recovery mechanisms
- Use strong typing for token categories
- Implement proper position tracking
- Add comprehensive documentation
- Follow LLVM lexer design patterns where applicable

4. Testing Requirements:
- Maintain existing test coverage
- Add performance benchmarks
- Include fuzz testing
- Test memory usage patterns
- Validate against the Delegua language specification

Please share the current lexer.rs implementation to receive specific optimization guidance. Reference materials:
- Delegua Wiki: https://github.com/DesignLiquido/delegua/wiki
- Rust Performance Book
- LLVM Lexer Documentation