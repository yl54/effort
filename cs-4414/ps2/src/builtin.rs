// File to deal with builtin handler

use cd;
use history;
use std::string::String;

pub struct BuiltIn {
    pub history: Vec<String>,
}

// CMD_LIST is a list of strings that are built in commands.
static CMD_LIST: [&'static str; 2] = [
    "cd",
    "history"
];

impl BuiltIn {
    // is_built_in checks if the command is built in.
    pub fn is_built_in(&self, input: &str) -> bool {
        // return the result of contains
        return self.contains(input);
    }

    // run runs the built in command.
    pub fn run(&self, cmd_name: &str, input: &str) -> bool {
        let mut success: bool = false;

        match cmd_name {
            "cd" => {
                success = cd::run(input);
            },
            "history" => {
                success = history::run(&self.history);
            },
            _ => {
                println!("No cmd actually exists for this one.");
            }
        }

        return success;
    }

    // record_cmd records the command in the history list.
    pub fn record_cmd(&mut self, s: String) {
        self.history.push(s);
    }

    // contains checks the built in list to see if it is a built in command.
    fn contains(&self, input: &str) -> bool {
        for i in 0..CMD_LIST.len() {
            if input == CMD_LIST[i] {
                return true;
            }
        }

        return false;
    }
}

pub trait BuiltInCommand {
    fn print(&self);
    fn run(&self) -> bool; 
}
