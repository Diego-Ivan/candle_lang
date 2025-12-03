use std::io::BufReader;

use crate::parser::{Parser, Statement};

mod parser;
mod token;
mod tokenizer;

fn statement_printer(statements: &[Statement]) {
    for stmt in statements {
        match stmt {
            Statement::Load(name) => println!("stmt: LOAD {};", name),
            Statement::Predict(name) => println!("stmt: PREDICT {};", name),
            Statement::Analyze(args) => println!("stmt: ANALYZE {};", args.join(", ")),
            Statement::Evaluate(args) => println!("stmt: EVALUATE {};", args.join(", ")),
            Statement::Select(sel) => match sel {
                crate::parser::Select::Identifier(id) => {
                    println!("stmt: SELECT (IDENTIFIER VARIANT) ID: {};", id)
                }
                crate::parser::Select::From(path) => {
                    println!("SELECT (FROM PATH VARIANT) PATH: '{}' ;", path)
                }
            },
            Statement::Train => println!("stmt: TRAIN;"),
            Statement::Init(map) => {
                let mut items: Vec<_> = map.iter().collect();
                items.sort_by_key(|(k, _)| *k);
                let body = items
                    .iter()
                    .map(|(k, v)| format!("[KEY]{}: [VALUE]{}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("INIT {{ {} }};", body);
            }
            Statement::Split(map) => {
                let mut items: Vec<_> = map.iter().collect();
                items.sort_by_key(|(k, _)| *k);
                let body = items
                    .iter()
                    .map(|(k, v)| format!("[KEY]{}: [VALUE]{}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("SPLIT {{ {} }};", body);
            }
        }
    }
}

fn tokens_from_input(input: &str) -> Vec<crate::token::Token> {
    let reader = BufReader::new(input.as_bytes());
    let tokenizer = crate::tokenizer::Tokenizer::new(reader);
    tokenizer.map(|r| r.unwrap()).collect()
}

fn main() {
    let input = "LOAD dataset;\nANALYZE field1, field2;\nSELECT result;";
    let tokens = tokens_from_input(input);
    let mut parser = Parser::new(tokens);
    let program = parser.program().unwrap();

    statement_printer(&program);
}
