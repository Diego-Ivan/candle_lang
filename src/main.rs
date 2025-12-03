use clap::Parser as ClapParser;
use std::{io::BufReader, path::PathBuf};

use crate::parser::{Parser, Statement};

mod parser;
mod token;
mod tokenizer;

fn print_statement_name(
    type_name: &str,
    name: &str,
    children: &[(&str, &str)],
    indent_level: usize,
) {
    println!("|({type_name} {name})");

    for (c_type, c_name) in children {
        print!("|");
        for _ in 0..indent_level + 1 {
            print!("---")
        }
        println!("({c_type} {c_name})");
    }
}

fn statement_printer(statements: &[Statement]) {
    println!("PROGRAM");
    for stmt in statements {
        match stmt {
            Statement::Load(name) => {
                print_statement_name("stmt", "LOAD", &[("id", name.as_ref())], 0)
            }
            Statement::Predict(name) => {
                print_statement_name("stmt", "PREDICT", &[("id", name.as_ref())], 0);
            }
            Statement::Analyze(args) => {
                let args: Vec<(&str, &str)> = args.iter().map(|arg| ("id", arg.as_ref())).collect();
                print_statement_name("stmt", "ANALYZE", &args, 0);
            }
            Statement::Evaluate(args) => {
                let args: Vec<(&str, &str)> = args.iter().map(|arg| ("id", arg.as_ref())).collect();
                print_statement_name("stmt", "EVALUATE", &args, 0);
            }
            Statement::Select(sel) => match sel {
                crate::parser::Select::Identifier(id) => {
                    print_statement_name("stmt", "SELECT", &[("id", id)], 0);
                }
                crate::parser::Select::From(path) => {
                    print_statement_name("stmt", "SELECT", &[("FROM", path)], 0);
                }
            },
            Statement::Train => print_statement_name("stmt", "TRAIN", &[], 0),
            Statement::Init(map) => {
                let items: Vec<(&str, String)> = map
                    .iter()
                    .map(|(key, value)| {
                        let entry_format = format!("key({key}), value({value})");
                        ("entry", entry_format)
                    })
                    .collect();

                let items: Vec<(&str, &str)> =
                    items.iter().map(|(a, b)| (*a, b.as_ref())).collect();
                print_statement_name("stmt", "INIT", &items, 0);
            }
            Statement::Split(map) => {
                let items: Vec<(&str, String)> = map
                    .iter()
                    .map(|(key, value)| {
                        let entry_format = format!("key({key}), value({value})");
                        ("entry", entry_format)
                    })
                    .collect();

                let items: Vec<(&str, &str)> =
                    items.iter().map(|(a, b)| (*a, b.as_ref())).collect();
                print_statement_name("stmt", "SPLIT", &items, 0);
            }
        }
    }
}

fn tokens_from_input(input: &str) -> Vec<crate::token::Token> {
    let reader = BufReader::new(input.as_bytes());
    let tokenizer = crate::tokenizer::Tokenizer::new(reader);
    tokenizer.map(|r| r.unwrap()).collect()
}

#[derive(ClapParser, Debug)]
#[command(name="pycandle", version = "0.5.0", about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Clone, Debug)]
enum Command {
    /// Generates the Abstract-Syntax-Tree for a Candle file.
    Ast { path: PathBuf },
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Ast { path: file_path } => {
            let input = std::fs::read_to_string(file_path).unwrap();

            let tokens = tokens_from_input(&input);
            let mut parser = Parser::new(tokens);

            let program = match parser.program() {
                Ok(stmts) => stmts,
                Err(e) => {
                    println!("Found an error while parsing code");
                    println!("{e}");
                    return;
                }
            };

            println!("---CÓDIGO ORIGINAL---");

            println!("{input}");

            println!("--------------------");

            println!("\n::AST::\n");

            statement_printer(&program);
        }
    }
}
