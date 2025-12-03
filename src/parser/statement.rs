use std::collections::HashMap;

#[derive(Debug)]
pub enum Statement {
    Load(String),
    Predict(String),
    Analyze(Vec<String>),
    Select(Select),
    Train,
    Evaluate(Vec<String>),
    Split(HashMap<String, f64>),
    Init(HashMap<String, f64>),
}

#[derive(Debug)]
pub enum Select {
    From(String),
    Identifier(String),
}
