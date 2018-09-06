// File to handle the `history` command

use builtin::BuiltInCommand;
use std::fmt::{Display, Formatter, Result};
use std::string::String;

// Struct to handle history
pub struct History<'a> {
    history: &'a Vec<String>,
}

impl <'a>History<'a> {
    pub fn new(h: &'a Vec<String>) -> Self {
        return History {
            history: h,
        };
    }
}

// Implement BuiltInCommand for history
impl<'a> BuiltInCommand for History <'a> {
    // print
    fn print(&self) {
        println!("{:?}", self.history)
    }

    // run
    fn run(&self) -> bool {
        // Loop through each element in the history.
        for record in self.history {
            // Print the record in the history
            println!("{}", record);
        }

        return true;
    }
}

// Implement Display for history
impl<'a> Display for History<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "History")
    }
}