// This file has the implementation of the gash shell.

use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::vec::Vec;

// CMD_LIST is a list of strings that are built in commands.
static CMD_LIST: [&'static str; 2] = [
    "cd",
    "history"
];

pub struct Shell<'a> {
    cmd_prompt: &'a str,

    // TODO: Make history an object here
    history_list: Vec<String>,

   // TODO: Add a mpsc here to send/recieve asynchronous stuff
   //       This will serve as the messages that are shown on the shell prompt
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { 
            cmd_prompt: prompt_str,
            history_list:    vec![],
        }
    }

    // run is the main loop for the gash shell.
    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Show the input prompt to the user
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            // Leave a blocking call for input.
            let mut raw_input = String::new();
            stdin.read_line(&mut raw_input).unwrap();
            let input = raw_input.trim();

            // Copy the input and record to history.
            // This provides the shell a dedicated copy that is not shared with commands. 
            // Copied from fletcher
            let historical_copy = input.clone();
            self.history_list.push(historical_copy.to_string());

            // Split the string by " "
            let spl: Vec<&str> = input.split(" ").collect();

            // Check if we need to exit
            match spl[0] {
                ""      =>  { continue; }
                "exit" => { return; }
                _ => { }
            }

            // Check if it is an asynchronous execution
            match spl[spl.len() - 1] {
                "&" => { self.run_cmd(spl); }
                _ => { self.run_cmd(spl); }
            }
        }
    }

    // is_built_in checks if the command is built in.
    fn is_built_in(&self, input: &str) -> bool {
        // return the result of contains
        return self.contains(input);
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

    // TODO: Add builtin stuff here
    // run runs the built in command.
    fn run_cmd(&self, argv: Vec<&str>) {
        match argv[0] {
            "cd" => self.cd(&argv[1..]),
            "history" => self.history(),
            _ => { self.run_custom_cmd(argv) }
        }
    }

    // run_custom_cmd runs the custom command passed in.
    fn run_custom_cmd(&self, argv: Vec<&str>) {
        if !self.path_cmd_exists(argv[0]) {
            println!("Custom command does not exist.");
            return;
        }

        match argv.first() {
            Some(&program) => {
                // At this point, its already been confirmed that the command exists on PATH.
                io::stdout().write(&Command::new(program).args(&argv[1..]).output().unwrap().stdout).unwrap();
            },
            None => (),
        };
    }

    // path_cmd_exists checks if the command exists on PATH.
    fn path_cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }

    fn cd(&self, _args: &[&str]) {
        let (dest, success) = match _args.len() {
            0 => { ("", true) },
            1 => { (_args[0], true) },
            _ => { ("", false) },      
        };

        if !success {
            // TODO: Add something
        }

        env::set_current_dir(dest).is_ok();
    }

    fn history(&self) {
        // Clone each history value dedicated for letting the println function borrow it.
        // Stolen from fletcher
        let mut results = vec![];
        for history_record in &self.history_list {
            results.push(history_record.clone());
        }
        println!("{}", format!("{:#?}", results));    
    }

    // TODO: Add an async command here
}
