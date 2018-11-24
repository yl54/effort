// This file has the implementation of the gash shell.

use builtin::BuiltIn;
use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::vec::Vec;

// GashAction describes what to do per input.
enum GashAction {
    Continue,
    Stop,
    RunSync,
    RunAsync,
    RunSyncBuiltIn,
    RunAsyncBuiltIn,
}

// CMD_LIST is a list of strings that are built in commands.
static CMD_LIST: [&'static str; 2] = [
    "cd",
    "history"
];

pub struct Shell<'a> {
    cmd_prompt: &'a str,

    // TODO: Make history an object here
    history: Vec<String>,

    // TODO: Make builtin a set of functions rather than an object.
    //       Or make it a part of the shell
    built_in:   BuiltIn,

   // TODO: Add a mpsc here to send/recieve asynchronous stuff
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { 
            cmd_prompt: prompt_str,
            history:    vec![],
            built_in:   BuiltIn { history: Vec::new() },           
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
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();

            // Copy the input and record to history.
            // This provides the shell a dedicated copy that is not shared with commands. 
            // Copied from fletcher
            let historical_copy = line.clone();
            self.history.push(historical_copy.to_string());

            // TODO: Figure out what `program` does.
            let cmd_line = line.trim();
            let program = cmd_line.splitn(1, " ").nth(0).expect("no program");
            println!("cmd_line: {}", cmd_line);
            println!("program: {}", program);

            // Split the string by " "
            let spl: Vec<&str> = cmd_line.split(" ").collect();

            // Check how the input should be handled.
            match self.check_cmd(program) {
                GashAction::Continue            => { continue; }
                GashAction::Stop                => { return; }
                GashAction::RunSync             => { self.run_custom_cmd(spl); }
                GashAction::RunAsync            => { self.run_custom_cmd(spl); }
                GashAction::RunSyncBuiltIn      => { self.built_in.run(spl[0], program); }
                GashAction::RunAsyncBuiltIn     => { self.built_in.run(spl[0], program); }
            }
        }
    }

    // check_cmd checks the input and determines the action to take.
    fn check_cmd(&mut self, raw_input: &str) -> GashAction {
        match raw_input {
            ""      =>  { return GashAction::Continue; }
            "exit"  =>  { return GashAction::Stop; }
            _       =>  {  }
        };

        let spl: Vec<&str> = raw_input.split(" ").collect();
        let len = spl.len();

        // Check if the array actually exists.
        if len <= 0 {
            println!("Input was not parsed into parts: {}", raw_input);
            return GashAction::Continue;
        }

        // Check if the command does not exist as a builtin or in the PATH.
        if !self.is_built_in(spl[0]) && !self.path_cmd_exists(raw_input) {
            println!("Input was not a built in or on PATH: {}", raw_input);
            return GashAction::Continue;
        }

        // Check if its a built in.
        if self.is_built_in(spl[0]) {
            match spl[len - 1] {
                "&" => { return GashAction::RunAsyncBuiltIn; }
                _ => { return GashAction::RunSyncBuiltIn; }
            }
        }

        match spl[len - 1] {
            "&" => { GashAction::RunAsync }
            _ => { GashAction::RunSync }
        }
    }

    // path_cmd_exists checks if the command exists on PATH.
    fn path_cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
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

    // run_custom_cmd runs the custom command passed in.
    fn run_custom_cmd(&self, argv: Vec<&str>) {
        match argv.first() {
            Some(&program) => {
                // At this point, its already been confirmed that the command exists on PATH.
                io::stdout().write(&Command::new(program).args(&argv[1..]).output().unwrap().stdout).unwrap();
            },
            None => (),
        };
    }

    // TODO: Add builtin stuff here

    // TODO: Add an async command here
}
