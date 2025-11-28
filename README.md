# Candle Language Tokenizer

A tokenizer implementation for the Candle programming language, a domain-specific language for machine learning operations.

## Overview

The Candle language tokenizer converts source code into a stream of tokens using a Deterministic Finite Automaton (DFA) approach. It provides detailed error reporting with position tracking (line and column numbers) for better debugging experience.

## Language Features

### Keywords
- `LOAD` - Load data
- `PREDICT` - Make predictions
- `TRAIN` - Train models
- `INIT` - Initialize
- `SPLIT` - Split data
- `ANALYZE` - Analyze data
- `EVALUATE` - Evaluate models
- `SELECT` - Select data
- `FROM` - Source specification

### Token Types
- **Keywords**: Reserved words for language operations (uppercase only: `LOAD`, `PREDICT`, etc.)
- **Identifiers**: Variable and function names (case-sensitive)
- **Numbers**: Integer and floating-point literals
- **Strings**: Text enclosed in single quotes (`'...'`)
- **Delimiters**: `{`, `}`, `;`, `,`, `:`

## Architecture

### Token Structure
```rust
pub struct Token {
    token_type: TokenType,
    column: usize,
    line: usize,
}
```

Each token contains:
- **token_type**: The type of token (keyword, identifier, number, etc.)
- **column**: Column position in source code
- **line**: Line number in source code

### DFA State Machine

The tokenizer uses a DFA with the following state transitions:

```
Start State → 
  ├─ Letter → Identifier/Keyword State
  ├─ Digit → Number State
  ├─ Quote → String State
  ├─ Delimiter → Single-char token
  ├─ Whitespace → Skip and continue
  └─ Unknown → Error
```

### Error Handling

The tokenizer provides detailed error messages including:
- **UnknownCharacter**: Unrecognized bytes with position (e.g., unsupported symbols like `.`, `@`, `#`)
- Error recovery: Tokenizer advances past unknown characters to continue processing

## Implementation Phases

### 🔧 Phase 4: Enhanced Errors
- [ ] Expand error types with position info
- [ ] Add contextual error messages
- [ ] Implement error recovery strategies

### 🔧 Phase 5: Iterator Implementation
- [ ] Complete `Iterator` trait for token streaming
- [ ] Handle EOF correctly

### 🔧 Phase 6: Testing
- [ ] Unit tests for individual components
- [ ] Integration tests for complete tokenization
- [ ] Error handling tests
- [ ] Position tracking validation
- [ ] Edge case tests

### 🔧 Phase 7: Public API
- [ ] Constructor methods
- [ ] `tokenize_all()` for batch processing
- [ ] Documentation and examples

## Usage Example

```rust
use candle_lang::tokenizer::Tokenizer;
use std::io::BufReader;

let source = "LOAD data FROM 'file.csv';";
let reader = BufReader::new(source.as_bytes());
let tokenizer = Tokenizer::new(reader);

for token in tokenizer {
    match token {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Testing Strategy

### Unit Tests
- Individual component testing (whitespace, identifiers, numbers, strings)
- Keyword matching verification
- Position tracking accuracy

### Integration Tests
- Complete statement tokenization
- Multi-line program parsing
- Mixed token sequences

### Error Tests
- Unknown character detection
- Unterminated string handling (strings ending with newline)
- Multiple errors in single input
- Position accuracy in errors

## Project Structure

```
candle_lang/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── token.rs          # Token types and structures
│   ├── tokenizer.rs      # Main tokenizer implementation
│   ├── tokenizer/
│   │   ├── error.rs      # Error types and handling
│   │   └── tests.rs      # Comprehensive test suite
│   └── parser.rs         # Parser (future)
└── target/               # Build artifacts
```

## Key Design Decisions

- **UTF-8 Support**: Properly handles multi-byte UTF-8 characters using Rust's `String` and `.chars()` iterator
- **Streaming**: Reads entire input as UTF-8 validated string for correct character handling
- **Iterator Pattern**: Lazy tokenization with automatic EOF handling
- **Position Tracking**: Line and column tracking for detailed error reporting (0-indexed columns)
- **Error Recovery**: Advances past unknown characters to continue tokenization instead of stopping
- **Case-Sensitive Keywords**: Only uppercase keywords (e.g., `LOAD`) are recognized; lowercase versions are identifiers

## Contributing

When implementing new features:
1. Write tests before implementation (TDD approach)
2. Ensure position tracking is accurate
3. Add error handling for edge cases
4. Update documentation
5. Run full test suite: `cargo test`

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output visible
cargo test -- --nocapture
```

## License

[Add your license here]

## Authors

Diego Ivan Martinez Escobar 

Azuany Mila Ceron
