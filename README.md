# Candle Language

Candle is a domain-specific language (DSL) for machine learning workflows, specifically designed for training and evaluating computer vision models like YOLO. It provides a simple, declarative syntax for common ML operations.

## Overview

This project includes a complete implementation of the Candle language with:
- **Tokenizer**: Converts source code into tokens using a DFA approach
- **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
- **CLI Tool**: `pycandle` command-line interface for parsing Candle scripts

## Language Features

### Keywords
- `LOAD` - Load a dataset
- `SELECT` - Select a model or configuration
- `FROM` - Specify a file path (used with SELECT)
- `SPLIT` - Define data split ratios
- `TRAIN` - Train the model
- `EVALUATE` - Evaluate model with metrics
- `PREDICT` - Generate predictions
- `ANALYZE` - Analyze results
- `INIT` - Initialize parameters

### Statements

#### LOAD
Load a dataset by name:
```candle
LOAD oxfordpets;
LOAD dota2025;
```

#### SELECT FROM
Select a model configuration file:
```candle
SELECT FROM 'yolov8m.yaml';
SELECT FROM 'rcnn.yaml';
```

#### SPLIT
Define dataset split ratios using a dictionary:
```candle
SPLIT { train: 80, test: 10, val: 10 };
SPLIT { train: 100 };
```

#### TRAIN
Train the model:
```candle
TRAIN;
```

#### EVALUATE
Evaluate with specified metrics:
```candle
EVALUATE mAP, recall, precision, AP50;
EVALUATE mAP50, recall;
```

#### PREDICT
Generate predictions:
```candle
PREDICT predictresults;
```

#### ANALYZE
Analyze results:
```candle
ANALYZE health, confusionmatrix;
ANALYZE health, splits;
```

### Token Types
- **Keywords**: Reserved words (uppercase only: `LOAD`, `PREDICT`, `TRAIN`, etc.)
- **Identifiers**: Variable and dataset names (case-sensitive, alphanumeric)
- **Numbers**: Integer and floating-point literals (e.g., `80`, `10.5`)
- **Strings**: Text enclosed in single quotes (`'yolov8m.yaml'`)
- **Delimiters**: `{`, `}`, `;`, `,`, `:`

## Complete Example

Here's a full Candle program demonstrating the workflow:

```candle
LOAD oxfordpets;
SELECT FROM 'yolov8m.yaml';

SPLIT {
  train: 80, 
  test: 10, 
  val: 10
};
TRAIN;

EVALUATE mAP, recall, precision, AP50;

PREDICT predictresults;
ANALYZE health, confusionmatrix;
```

This program:
1. Loads the Oxford Pets dataset
2. Selects the YOLOv8m model configuration
3. Splits data into 80% training, 10% test, 10% validation
4. Trains the model
5. Evaluates using multiple metrics
6. Generates predictions
7. Analyzes model health and confusion matrix

## Architecture

### Token Structure
```rust
pub struct Token {
    token_type: TokenType,
    column: usize,
    line: usize,
}
```

Each token tracks:
- **token_type**: The classification (keyword, identifier, number, string, delimiter)
- **column**: Column position in source code
- **line**: Line number in source code

### Statement Types

The parser generates an AST with the following statement types:

```rust
pub enum Statement {
    Load(String),                      // Dataset name
    Predict(String),                   // Output name
    Analyze(Vec<String>),              // List of metrics
    Evaluate(Vec<String>),             // List of metrics
    Select(Select),                    // Model selection
    Train,                             // No arguments
    Split(HashMap<String, f64>),       // Split ratios
    Init(HashMap<String, f64>),        // Parameters
}

pub enum Select {
    From(String),        // File path
    Identifier(String),  // Model name
}
```

### Tokenizer DFA State Machine

The tokenizer implements a DFA with state transitions:

```
Start State → 
  ├─ Letter → Identifier/Keyword State
  ├─ Digit → Number State (supports decimals with '.')
  ├─ Single Quote (') → String State
  ├─ Delimiter ({, }, ;, ,, :) → Single-char token
  ├─ Whitespace → Skip and continue
  └─ Unknown → Error
```

### Parser Grammar

The parser implements a recursive descent parser with the following rules:

```
program        → statement* EOF
statement      → loadStmt | predictStmt | analyzeStmt | evaluateStmt 
                 | trainStmt | selectStmt | splitStmt | initStmt
loadStmt       → "LOAD" identifier ";"
predictStmt    → "PREDICT" identifier ";"
analyzeStmt    → "ANALYZE" argumentList ";"
evaluateStmt   → "EVALUATE" argumentList ";"
trainStmt      → "TRAIN" ";"
selectStmt     → "SELECT" (identifier | "FROM" string) ";"
splitStmt      → "SPLIT" dictionary ";"
initStmt       → "INIT" dictionary ";"
argumentList   → identifier ("," identifier)*
dictionary     → "{" identifier ":" number ("," identifier ":" number)* "}"
```

### Error Handling

**Tokenizer Errors:**
- `UnknownCharacter`: Invalid characters with position tracking
- `UnterminatedString`: Strings not closed before newline or EOF
- `InvalidNumber`: Malformed numeric literals

**Parser Errors:**
- `ExpectedId`: Missing identifier
- `ExpectedSemicolon`: Missing statement terminator
- `ExpectedColon`: Missing colon in dictionary
- `ExpectedNumber`: Missing numeric value
- `InvalidStatement`: Unrecognized statement keyword
- `NotADictionary`: Missing opening brace
- `NonTerminatedDictionary`: Missing closing brace
- `NoStringAfterFrom`: Missing file path after FROM
- `InvalidSelect`: Invalid SELECT syntax

Both provide line and column information for debugging.

## Installation & Usage

### Building from Source

```bash
cargo build --release
```

### Running the CLI

The `pycandle` CLI tool parses Candle files and displays their AST:

```bash
pycandle ast <path-to-candle-file>
```

**Example:**
```bash
pycandle ast examples/valid/script.candle
```

**Output:**
```
---CÓDIGO ORIGINAL---
LOAD oxfordpets;
SELECT FROM 'yolov8m.yaml';

SPLIT {
  train: 80, 
  test: 10, 
  val: 10
};
TRAIN;

EVALUATE mAP, recall, precision, AP50;

PREDICT predictresults;
ANALYZE health, confusionmatrix;
--------------------

::AST::

PROGRAM
|(stmt LOAD)
|---(id oxfordpets)
|(stmt SELECT)
|---(FROM yolov8m.yaml)
|(stmt SPLIT)
|---(entry key(train), value(80))
|---(entry key(test), value(10))
|---(entry key(val), value(10))
|(stmt TRAIN)
|(stmt EVALUATE)
|---(id mAP)
|---(id recall)
|---(id precision)
|---(id AP50)
|(stmt PREDICT)
|---(id predictresults)
|(stmt ANALYZE)
|---(id health)
|---(id confusionmatrix)
```

## Library Usage

### Tokenizer

```rust
use std::io::BufReader;
use candle_lang::tokenizer::Tokenizer;

let source = "LOAD dataset; TRAIN;";
let reader = BufReader::new(source.as_bytes());
let tokenizer = Tokenizer::new(reader);

for token_result in tokenizer {
    match token_result {
        Ok(token) => println!("{:?}", token),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Parser

```rust
use candle_lang::parser::Parser;
use candle_lang::tokenizer::Tokenizer;
use std::io::BufReader;

let source = "LOAD oxfordpets; TRAIN; EVALUATE mAP;";
let reader = BufReader::new(source.as_bytes());
let tokenizer = Tokenizer::new(reader);
let tokens: Vec<_> = tokenizer.map(|r| r.unwrap()).collect();

let mut parser = Parser::new(tokens);
match parser.program() {
    Ok(statements) => {
        for stmt in statements {
            println!("{:?}", stmt);
        }
    }
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## Testing

The project includes comprehensive tests for both tokenizer and parser:

### Running Tests

```bash
cargo test
```

### Tokenizer Tests
- **Unit tests**: Individual token recognition (whitespace, identifiers, keywords, numbers, strings)
- **Keyword matching**: Case-sensitive keyword detection (uppercase only)
- **Position tracking**: Accurate line and column reporting
- **Error handling**: Unknown characters, unterminated strings
- **Multi-line programs**: Proper line tracking across newlines

### Parser Tests
- **Statement parsing**: All statement types (LOAD, TRAIN, EVALUATE, etc.)
- **Dictionary parsing**: SPLIT and INIT with key-value pairs
- **Argument lists**: ANALYZE and EVALUATE with comma-separated identifiers
- **SELECT variations**: Both identifier and FROM forms
- **Error detection**: Missing semicolons, invalid syntax, unterminated dictionaries
- **Complete programs**: Full multi-statement programs

## Examples

The `examples/valid/` directory contains sample Candle programs:

- **01.candle**: Basic dataset loading and splitting
- **02.candle**: Dataset loading with analysis and training
- **03.candle**: Training with evaluation metrics
- **04.candle**: Model selection from YAML with evaluation
- **script.candle**: Complete ML workflow with all features

## Project Structure

```
candle_lang/
├── Cargo.toml              # Project configuration
├── README.md               # This file
├── examples/
│   └── valid/              # Example Candle programs
│       ├── 01.candle
│       ├── 02.candle
│       ├── 03.candle
│       ├── 04.candle
│       ├── 05.candle
│       └── script.candle
└── src/
    ├── main.rs             # CLI entry point
    ├── token.rs            # Token types and structure
    ├── tokenizer.rs        # Lexical analysis
    ├── tokenizer/
    │   ├── error.rs        # Tokenizer error types
    │   └── tests.rs        # Tokenizer unit tests
    ├── parser.rs           # Syntax analysis
    └── parser/
        ├── error.rs        # Parser error types
        ├── statement.rs    # AST statement types
        └── tests.rs        # Parser unit tests
```

## License

This project is part of a Theory of Computation course project.

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
