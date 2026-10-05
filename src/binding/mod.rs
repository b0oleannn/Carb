use std::process::exit;

use colored::{ColoredString, Colorize};

pub mod binder;
pub mod values;

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Binder Error".red().bold(),
    );
    exit(-1);
}
