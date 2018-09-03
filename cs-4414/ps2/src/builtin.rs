// File to deal with builtin handler

use cd::Cd;
use std::string::String;

#[allow(dead_code)] 
#[allow(unused_imports)]

// mod cd;

// Builtin struct
pub struct BuiltIn {
}

// List of strings that represent builtin
// Make this a simple list first
// Maybe this is a hashmap or something later
static CMD_LIST: [&'static str; 2] = [
    "cd",
    "history"
];

impl BuiltIn {
    // Function to check if its a builtin or not
    pub fn is_built_in(&self, input: &str) -> bool {
        // return the result of contains
        return self.contains(input);
    }

    // Function to return a cmd.
    pub fn get_built_in_cmd(&self, input: &str) -> impl BuiltInCommand {
        match input {
            "cd" => Cd::new(input.to_string()),
            _    => Cd::new(input.to_string()),
        }
    }

    // Function to check if the list contains the str
    fn contains(&self, input: &str) -> bool {
        // Loop through each of the string
        for i in 0..CMD_LIST.len() {
            // If its known, return true;
            if input == CMD_LIST[i] {
                return true;
            }
        }

        // All have been searched through. Return false
        return false;
    }
}

// cmd trait definition
// - parse args
// - run
pub trait BuiltInCommand {
    fn new(String) -> Self;

    fn print(&self);

    // Function to run the command.
    // Return true if command successfully runs.
    fn run(&self) -> bool; 
}
