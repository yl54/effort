// File to handle the `history` command

use builtin::BuiltInCommand;
use std::fmt::{Display, Formatter, Result};
use std::string::String;

pub fn run(history: &Vec<String>) -> bool {
    for record in history {
        println!("{}", record);
    }

    return true;
}