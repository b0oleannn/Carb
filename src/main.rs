use std::fs;

use crate::{
    binding::binder,
    frontend::parser::{parser::Parser, pretty_bounded_print, pretty_print_ast},
    runtime::{evaluator::evaluate_program, values::Program},
};
pub mod binding;
pub mod frontend;
pub mod runtime;
// TODO: Strings, comments.
fn main() {
    let file_path = "/home/b0olean/RustroverProjects/Carb/main.carb";
    evaluate_file(file_path);
}

pub fn evaluate_file(file_path: &str) {
    let input = fs::read_to_string(file_path).unwrap();
    let mut parser = Parser::new(input);
    let ast = parser.produce_ast();
    pretty_print_ast(&ast);

    let mut bounds = vec![];

    for statement in ast {
        bounds.push(binder::bind_statement(statement));
    }
    let program = Program::new(bounds);
    pretty_bounded_print(&program);

    evaluate_program(program)
}
