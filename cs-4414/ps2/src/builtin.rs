// File to deal with builtin handler

use cd::Cd;
use history::History;
use std::string::String;

// mod cd;

// Builtin struct
pub struct BuiltIn {
    // Lesson: you don't need to make each field mutable.
    //         you have to make the entire struct mutable.
    pub history: Vec<String>,
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
    // Return if the run was successful
    pub fn run(&self, cmd_name: &str, input: &str) -> bool {
        let mut success: bool = false;

        match cmd_name {
            "cd" => {
                let cmd: Cd = Cd::new(input.to_string());
                success = cmd.run();
            },
            "history" => {
                let cmd: History = History::new(&self.history);
                success = cmd.run();
            },
            _ => {
                println!("No cmd actually exists for this one.");
            }
        }

        return success;
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

    // Function to record history.
    pub fn record_cmd(&mut self, s: String) {
        self.history.push(s);
    }
}

// cmd trait definition
// - parse args
// - run
pub trait BuiltInCommand {
    fn print(&self);

    // Function to run the command.
    // Return true if command successfully runs.
    fn run(&self) -> bool; 
}
