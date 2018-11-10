// This file has the implementation of the gash shell.

use builtin::BuiltIn;
use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::vec::Vec;

// Add a list of Gash actions: 1
enum GashAction {
    Continue,
    Stop,
    RunSync,
    RunAsync,
    RunSyncBuiltIn,
    RunAsyncBuiltIn,
}

pub struct Shell<'a> {
    cmd_prompt: &'a str,
    built_in:   BuiltIn,
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { 
            cmd_prompt: prompt_str,
            built_in:   BuiltIn { history: Vec::new() },           
        }
    }

    // run is the main loop for the gash shell.
    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            let mut line = String::new();

            stdin.read_line(&mut line).unwrap();
            let cmd_line = line.trim();
            let program = cmd_line.splitn(1, " ").nth(0).expect("no program");
   
            println!("cmd_line: {}", cmd_line);
            println!("program: {}", program);

            // Record every command that comes in.
            self.built_in.record_cmd(program.to_string());

            // Split the string by " "
            let spl: Vec<&str> = cmd_line.split(" ").collect();

            // Check how the input should be handled.
            match self.check_cmd(program) {
                GashAction::Continue            => { continue; }
                GashAction::Stop                => { return; }
                GashAction::RunSync             => { 
                    self.run_cmd(spl[0], program);
                }
                GashAction::RunAsync            => { 
                    self.run_cmd(spl[0], program);
                }
                GashAction::RunSyncBuiltIn      => { return; }
                GashAction::RunAsyncBuiltIn     => { return; }
            }
        }
    }

    // check_cmd checks the input and decides action to take.
    fn check_cmd(&mut self, cmd_line: &str) -> GashAction {
        match cmd_line {
            ""      =>  { return GashAction::Continue; }
            "exit"  =>  { return GashAction::Stop; }
            _       =>  {  }
        };

        let spl: Vec<&str> = cmd_line.split(" ").collect();
        let len = spl.len();

        // Check if the array actually exists.
        if len <= 0 {
            println!("Input was not parsed into parts: {}", cmd_line);
            return GashAction::Continue;
        }

        // Check if the command does not exist as a builtin or in the PATH.
        if !self.built_in.is_built_in(spl[0]) && !self.path_cmd_exists(cmd_line) {
            println!("Input was not a built in or on PATH: {}", cmd_line);
            return GashAction::Continue;
        }

        // Check if its a built in.
        if self.built_in.is_built_in(spl[0]) {
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

    // run_cmd runs the command.
    fn run_cmd(&self, cmd_name: &str, cmd_line: &str) {
        // Check if it is a builtin
        if self.built_in.is_built_in(cmd_name) {
            println!("BuiltIn command spotted from: {}", cmd_line);

            // Run the builtin command
            let success: bool = self.built_in.run(cmd_name, cmd_line);
            match success {
                true => { println!("Successful builtin command."); }
                false => { println!("Failed builtin command."); }
            }
        } else {
            self.run_custom_cmdline(cmd_line);
        }
    }

    // run_custom_cmdline runs the custom command.
    pub fn run_custom_cmdline(&self, cmd_line: &str) {
        let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x)
            }
        }).collect();

        match argv.first() {
            Some(&program) => {
                // At this point, its already been confirmed that the command exists on PATH.
                io::stdout().write(&Command::new(program).args(&argv[1..]).output().unwrap().stdout).unwrap();
            },
            None => (),
        };
    }    
}