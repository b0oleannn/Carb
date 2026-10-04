use std::io;

use crate::frontend::parser::{parser::Parser, pretty_print};
pub mod frontend;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read the user`s input");
        //println!("input: {input:?}");
        let mut parser = Parser::new(input);
        for statement in parser.parse() {
            pretty_print(statement);
        }
    }
}
