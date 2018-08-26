// File to deal with builtin handler

use std::vec::Vec;

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

// cmd trait definition
// - parse args
// - run
trait InternalCommand {
    // Function to parse arguments.
    // Return true if parsing is successful
    fn parse_args(&self, &str) -> bool;

    // Function to run the command.
    // Return true if command successfully runs.
    fn run(&self) -> bool; 
}

impl BuiltIn {
    // Function to check if its a builtin or not
    pub fn is_built_in(&self, full_input: &str) -> bool{
        // Split the string by " "
        let spl: Vec<&str> = full_input.split(" ").collect();

        // Check if the array actually exists.
        if spl.len() <= 0 {
            return false;
        }

        // Extract the first str
        let input: &str = spl[0];

        // return the result of contains
        return self.contains(input);
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
