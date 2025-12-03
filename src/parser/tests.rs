use super::*;
use std::io::BufReader;

fn tokens_from_input(input: &str) -> Vec<crate::token::Token> {
    let reader = BufReader::new(input.as_bytes());
    let tokenizer = crate::tokenizer::Tokenizer::new(reader);
    tokenizer.map(|r| r.unwrap()).collect()
}

#[test]
fn test_parse_load() {
    let input = "LOAD data;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Load(name) => assert_eq!(name, "data"),
        other => panic!("Unexpected statement: {:?}", other),
    }
}

#[test]
fn test_parse_select_identifier() {
    let input = "SELECT column;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Select(select) => match select {
            Select::Identifier(id) => assert_eq!(id, "column"),
            _ => panic!("Expected Identifier select"),
        },
        other => panic!("Unexpected statement: {:?}", other),
    }
}

#[test]
fn test_parse_select_from_string() {
    let input = "SELECT FROM 'file.csv';";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Select(select) => match select {
            Select::From(path) => assert_eq!(path, "file.csv"),
            _ => panic!("Expected From select"),
        },
        other => panic!("Unexpected statement: {:?}", other),
    }
}

#[test]
fn test_parse_analyze_arguments() {
    let input = "ANALYZE a, b, c;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Analyze(args) => {
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], "a");
            assert_eq!(args[1], "b");
            assert_eq!(args[2], "c");
        }
        other => panic!("Unexpected statement: {:?}", other),
    }
}

#[test]
fn test_parse_init_dictionary() {
    let input = "INIT { layers : 3 , epochs : 100 };";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::Init(map) => {
            assert_eq!(map.get("layers"), Some(&3.0));
            assert_eq!(map.get("epochs"), Some(&100.0));
        }
        other => panic!("Unexpected statement: {:?}", other),
    }
}

#[test]
fn test_invalid_statement_error() {
    let input = "FOO bar;"; // unknown starting token should be treated as Id and produce InvalidStatement
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let result = parser.program();
    assert!(result.is_err());
}

#[test]
fn test_parse_multiple_lines_two_statements() {
    let input = "LOAD data;\nTRAIN;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 2);

    match &program[0] {
        Statement::Load(name) => assert_eq!(name, "data"),
        other => panic!("Unexpected first statement: {:?}", other),
    }

    match &program[1] {
        Statement::Train => {}
        other => panic!("Unexpected second statement: {:?}", other),
    }
}

#[test]
fn test_parse_multiple_lines_various_statements() {
    let input = "LOAD dataset;\nANALYZE field1, field2;\nSELECT result;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser { tokens, current: 0 };
    let program = parser.program().unwrap();
    assert_eq!(program.len(), 3);

    match &program[0] {
        Statement::Load(name) => assert_eq!(name, "dataset"),
        other => panic!("Unexpected stmt 0: {:?}", other),
    }

    match &program[1] {
        Statement::Analyze(args) => {
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], "field1");
            assert_eq!(args[1], "field2");
        }
        other => panic!("Unexpected stmt 1: {:?}", other),
    }

    match &program[2] {
        Statement::Select(sel) => match sel {
            Select::Identifier(id) => assert_eq!(id, "result"),
            _ => panic!("Expected Identifier select"),
        },
        other => panic!("Unexpected stmt 2: {:?}", other),
    }
}
