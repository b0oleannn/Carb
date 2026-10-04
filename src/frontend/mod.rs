use std::process::exit;

use colored::{ColoredString, Colorize};

pub mod lexer;
pub mod parser;

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Frontend Error".red().bold(),
    );
    exit(-1);
}
