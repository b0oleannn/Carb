use std::process::exit;

use colored::{ColoredString, Colorize};

pub mod bind_binary_expression;
pub mod bind_unary_expression;
pub mod binder;
pub mod values;

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Binder Error".red().bold(),
    );
    exit(-1);
}
