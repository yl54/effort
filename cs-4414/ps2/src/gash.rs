// This file has the implementation of the gash shell.

use builtin::{BuiltIn, BuiltInCommand};
use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::vec::Vec;

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

            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                _       =>  { self.run_cmd(cmd_line) }
            }
        }
    }

    // Maybe I need a validate_cmd to check if:
    // - it can split into multiple strings
    // - to run as a separate thread or on main process

    fn run_cmd(&mut self, cmd_line: &str) {
        // Split the string by " "
        let spl: Vec<&str> = cmd_line.split(" ").collect();

        // Check if the array actually exists.
        if spl.len() <= 0 {
            println!("Input was not parsed into parts: {}", cmd_line);
            return;
        }

        // Extract the first str
        let input: &str = spl[0];

        // Check if it is a builtin
        if self.built_in.is_built_in(input) {
            println!("BuiltIn command spotted from: {}", cmd_line);

            // Run the builtin command
            let success: bool = self.built_in.run(input, cmd_line);
            match success {
                true => { println!("Successful builtin command."); }
                false => { println!("Failed builtin command."); }
            }
        } else {
            self.run_custom_cmdline(cmd_line);
        }

        self.built_in.record_cmd(cmd_line.to_string());
    }

    pub fn run_custom_cmdline(&self, cmd_line: &str) {
        let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x)
            }
        }).collect();

        match argv.first() {
            Some(&program) => self.run_custom_cmd(program, &argv[1..]),
            None => (),
        };
    }

    fn run_custom_cmd(&self, program: &str, argv: &[&str]) {
        if self.cmd_exists(program) {
            io::stdout().write(&Command::new(program).args(argv).output().unwrap().stdout).unwrap();
        } else {
            println!("{}: command not found", program);
        }
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}