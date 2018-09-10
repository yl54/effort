// File to handle the `cd` command

use builtin::BuiltInCommand;
use std::fmt::{Display, Formatter, Result};
use std::env;
use std::path::{Path};
use std::string::String;

// CD struct
pub struct Cd {
    dest: String,
    valid: bool,
}

impl Cd {
    pub fn new(input: String) -> Self {
        let spl: Vec<&str> = input.split(" ").collect();

        println!("spl: {:?}", spl);

        let (att_dest, success) = match spl.len() {
            0 => { ("", false) },
            1 => { ("", true) },
            2 => { (spl[1], true) },
            _ => { ("", false) },      
        };

        return Cd {
            dest: att_dest.to_string(), 
            valid: success,
        };
    }
}

impl BuiltInCommand for Cd { 
    fn print(&self) {
        println!("{}", self);
    }

    fn run(&self) -> bool {
        // Set the current working directory.
        return env::set_current_dir(&self.dest).is_ok();
    }
}

impl Display for Cd {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Cd:\n dest: {}\nvalid: {}\n", self.dest, self.valid)
    }
}
