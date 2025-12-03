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

fn tokens_from_input(input: &str) -> Result<Vec<crate::token::Token>, crate::tokenizer::TokenizerError> {
    let reader = BufReader::new(input.as_bytes());
    let tokenizer = crate::tokenizer::Tokenizer::new(reader);
    tokenizer.collect()
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

            let tokens = match tokens_from_input(&input) {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("Se encontró un error al tokenizar el código:");
                    println!("{e}");

                    println!("---CÓDIGO ORIGINAL---");
                    println!("{input}");
                    println!("--------------------");

                    return;
                }
            };
            let mut parser = Parser::new(tokens);

            let program = match parser.program() {
                Ok(stmts) => stmts,
                Err(e) => {
                    println!("An error was found while parsing the code:");
                    println!("{e}");
                    // Print where the error happened in the code
                    let error_line = input
                        .lines()
                        .nth(e.token.get_line() - 1)
                        .unwrap_or("<Could not retrieve line>");
                    // println!("Line {}:{}: {}", e.token.get_line(), e.token.get_column(), error_line);
                    println!("{error_line}");
                    
                    // Print column indicator
                    if e.token.get_column() > 0 {
                        let indicator = format!("{}^", " ".repeat(e.token.get_column() - 1));
                        println!("{}", indicator);
                    }

                    println!("\n---ORIGINAL CODE---");
                    println!("{input}");
                    return;
                }
            };

            println!("---ORIGINAL CODE---");

            println!("{input}");

            println!("--------------------");

            println!("\n::AST::\n");

            statement_printer(&program);
        }
    }
}
