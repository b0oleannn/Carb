use std::fs;

use crate::frontend::parser::{parser::Parser, pretty_print};
pub mod frontend;

fn main() {
    let file_path = "/home/b0olean/RustroverProjects/Carb/main.carb";
    evaluate_file(file_path);
}

pub fn evaluate_file(file_path: &str) {
    let input = fs::read_to_string(file_path).unwrap();
    let mut parser = Parser::new(input);
    let parsed = parser.parse();
    println!("Raw AST: {:?}", parsed);
    pretty_print(parsed);
}
