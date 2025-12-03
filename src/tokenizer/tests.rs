#[cfg(test)]
use crate::tokenizer::*;
use std::io::BufReader;

fn tokenize_input(input: &str) -> Vec<TokenizerResult<Token>> {
    let reader = BufReader::new(input.as_bytes());
    let tokenizer = Tokenizer::new(reader);
    tokenizer.collect()
}

// ===== Unit Tests =====

#[test]
fn test_skip_whitespace() {
    let input = "   \t\n  LOAD";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Load
    ));
}

#[test]
fn test_single_keyword() {
    let keywords = vec![
        ("LOAD", TokenType::Load),
        ("PREDICT", TokenType::Predict),
        ("TRAIN", TokenType::Train),
        ("INIT", TokenType::Init),
        ("SPLIT", TokenType::Split),
        ("ANALYZE", TokenType::Analyze),
        ("EVALUATE", TokenType::Evaluate),
        ("SELECT", TokenType::Select),
        ("FROM", TokenType::From),
    ];

    for (input, expected_type) in keywords {
        let tokens = tokenize_input(input);
        assert_eq!(tokens.len(), 1, "Failed for keyword: {}", input);
        let token = tokens[0].as_ref().unwrap();
        assert!(
            std::mem::discriminant(token.get_type()) == std::mem::discriminant(&expected_type),
            "Failed for keyword: {}",
            input
        );
    }
}

#[test]
fn test_identifiers() {
    let inputs = vec!["myVar", "data123", "pr1v4t3", "CamelCase"];

    for input in inputs {
        let tokens = tokenize_input(input);
        assert_eq!(tokens.len(), 1, "Failed for identifier: {}", input);
        let token = tokens[0].as_ref().unwrap();
        if let TokenType::Id(name) = token.get_type() {
            assert_eq!(name, input);
        } else {
            panic!("Expected Id token for: {}", input);
        }
    }
}

#[test]
fn test_numbers() {
    let test_cases = vec![("42", 42.0), ("3.1", 3.1), ("0", 0.0), ("123.456", 123.456)];

    for (input, expected) in test_cases {
        let tokens = tokenize_input(input);
        assert_eq!(tokens.len(), 1, "Failed for number: {}", input);
        let token = tokens[0].as_ref().unwrap();
        if let TokenType::Number(n) = token.get_type() {
            assert_eq!(*n, expected);
        } else {
            panic!("Expected Number token for: {}", input);
        }
    }
}

#[test]
fn test_strings_single_quote() {
    let input = "'hello world'";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 1);
    let token = tokens[0].as_ref().unwrap();
    if let TokenType::String(s) = token.get_type() {
        assert_eq!(s, "hello world");
    } else {
        panic!("Expected String token");
    }
}

#[test]
fn test_delimiters() {
    let delimiters = vec![
        (";", TokenType::Semicolon),
        (",", TokenType::Comma),
        (":", TokenType::Colon),
        ("{", TokenType::LeftBrace),
        ("}", TokenType::RightBrace),
    ];

    for (input, expected_type) in delimiters {
        let tokens = tokenize_input(input);
        assert_eq!(tokens.len(), 1, "Failed for delimiter: {}", input);
        let token = tokens[0].as_ref().unwrap();
        assert!(
            std::mem::discriminant(token.get_type()) == std::mem::discriminant(&expected_type),
            "Failed for delimiter: {}",
            input
        );
    }
}

// ===== Integration Tests =====

#[test]
fn test_simple_command() {
    let input = "LOAD data;";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 3);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Load
    ));
    assert!(matches!(
        tokens[1].as_ref().unwrap().get_type(),
        TokenType::Id(_)
    ));
    assert!(matches!(
        tokens[2].as_ref().unwrap().get_type(),
        TokenType::Semicolon
    ));
}

#[test]
fn test_complex_statement() {
    let input = "INIT model : { layers : 3 , epochs : 100 };";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 13);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Init
    ));
    assert!(matches!(
        tokens[1].as_ref().unwrap().get_type(),
        TokenType::Id(_)
    ));
    assert!(matches!(
        tokens[2].as_ref().unwrap().get_type(),
        TokenType::Colon
    ));
    assert!(matches!(
        tokens[3].as_ref().unwrap().get_type(),
        TokenType::LeftBrace
    ));
}

#[test]
fn test_multiline_program() {
    let input = "LOAD data;\nTRAIN model;\nPREDICT result;";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 9);

    // Check line numbers
    assert_eq!(tokens[0].as_ref().unwrap().get_line(), 1);
    assert_eq!(tokens[3].as_ref().unwrap().get_line(), 2);
    assert_eq!(tokens[6].as_ref().unwrap().get_line(), 3);
}

#[test]
fn test_mixed_tokens() {
    let input = "SELECT data FROM 'file.csv';";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 5);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Select
    ));
    assert!(matches!(
        tokens[1].as_ref().unwrap().get_type(),
        TokenType::Id(_)
    ));
    assert!(matches!(
        tokens[2].as_ref().unwrap().get_type(),
        TokenType::From
    ));
    assert!(matches!(
        tokens[3].as_ref().unwrap().get_type(),
        TokenType::String(_)
    ));
    assert!(matches!(
        tokens[4].as_ref().unwrap().get_type(),
        TokenType::Semicolon
    ));
}

// ===== Error Handling Tests =====

#[should_panic]
#[test]
fn test_unknown_character() {
    let input = "@";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 1);
    tokens[0].as_ref().unwrap();
}

#[test]
#[should_panic]
fn test_unterminated_string() {
    let input = "'hello\n";
    let tokens = tokenize_input(input);

    assert_eq!(tokens.len(), 1);
    tokens[0].as_ref().unwrap();
}

#[test]
#[should_panic]
fn test_multiple_errors() {
    let input = "LOAD @ data # test;";
    let tokens = tokenize_input(input);

    // Should have LOAD, then error, then data, then error, then Id(test), then semicolon
    for token in tokens {
        token.unwrap();
    }
}

// ===== Position Tracking Tests =====

#[test]
fn test_column_tracking() {
    let input = "LOAD data;";
    let tokens = tokenize_input(input);

    assert_eq!(tokens[0].as_ref().unwrap().get_column(), 0); // LOAD starts at column 1
    assert_eq!(tokens[1].as_ref().unwrap().get_column(), 5); // data starts at column 6
    assert_eq!(tokens[2].as_ref().unwrap().get_column(), 9); // ; at column 10
}

#[test]
fn test_line_tracking_multiple_lines() {
    let input = "LOAD\ndata\n;";
    let tokens = tokenize_input(input);

    assert_eq!(tokens[0].as_ref().unwrap().get_line(), 1);
    assert_eq!(tokens[1].as_ref().unwrap().get_line(), 2);
    assert_eq!(tokens[2].as_ref().unwrap().get_line(), 3);
}

// ===== Edge Case Tests =====

#[test]
fn test_empty_input() {
    let input = "";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 0); // Iterator returns None for EOF
}

#[test]
fn test_only_whitespace() {
    let input = "   \n\t\n  ";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_number_followed_by_identifier() {
    let input = "123abc";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 2);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Number(_)
    ));
    assert!(matches!(
        tokens[1].as_ref().unwrap().get_type(),
        TokenType::Id(_)
    ));
}

#[test]
fn test_decimal_without_trailing_digits() {
    let input = "123.;";
    let tokens = tokenize_input(input);

    // Should parse as: Number(123), Error('.'), Semicolon
    assert_eq!(tokens.len(), 3);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Number(123.0)
    ));
    assert!(tokens[1].is_err()); // '.' is unknown character
    assert!(matches!(
        tokens[2].as_ref().unwrap().get_type(),
        TokenType::Semicolon
    ));
}

#[test]
fn test_consecutive_delimiters() {
    let input = "{}:,;";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_keyword_case_sensitivity() {
    // Only uppercase keywords should be recognized
    let input = "LOAD";
    let tokens = tokenize_input(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(
        tokens[0].as_ref().unwrap().get_type(),
        TokenType::Load
    ));

    // Lowercase and mixed case should be treated as identifiers
    let non_keywords = vec!["load", "Load", "LoAd"];
    for input in non_keywords {
        let tokens = tokenize_input(input);
        assert_eq!(tokens.len(), 1, "Failed for: {}", input);
        assert!(
            matches!(tokens[0].as_ref().unwrap().get_type(), TokenType::Id(_)),
            "Expected Id for: {}",
            input
        );
    }
}
