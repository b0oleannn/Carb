use std::fs;

use crate::{
    binding::binder,
    evaluator::Program,
    frontend::parser::{parser::Parser, pretty_print},
};
pub mod binding;
pub mod evaluator;
pub mod frontend;

fn main() {
    let file_path = "/home/b0olean/RustroverProjects/Carb/main.carb";
    evaluate_file(file_path);
}

pub fn evaluate_file(file_path: &str) {
    let input = fs::read_to_string(file_path).unwrap();
    let mut parser = Parser::new(input);
    let ast = parser.produce_ast();
    println!("Raw Ast: {ast:?}");
    let mut bounds = vec![];

    for expr in ast {
        bounds.push(binder::bind_expression(expr));
    }
    let program = Program::new(bounds);
    pretty_print(&program);

    evaluator::evaluate_program(program)
}
