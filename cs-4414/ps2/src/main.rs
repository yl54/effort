//
// gash.rs
//
// Starting code for PS2
// Running on Rust 1+
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

#![allow(dead_code)] 
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]

extern crate getopts;

use builtin::{BuiltIn, BuiltInCommand};
use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::vec::Vec;

pub mod builtin;
pub mod cd;
pub mod history;

struct Shell<'a> {
    cmd_prompt: &'a str,
}

impl <'a>Shell<'a> {
    fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { cmd_prompt: prompt_str }
    }

    fn run(&self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        // Init the builtin here
        let mut built_in: BuiltIn = BuiltIn{
            history: Vec::new(),
        };

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
                _       =>  { self.run_cmd(&mut built_in, cmd_line) }
            }
        }
    }

    fn run_cmd(&self, built_in: &mut BuiltIn, cmd_line: &str) {
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
        if built_in.is_built_in(input) {
            println!("BuiltIn command spotted from: {}", cmd_line);

            // Run the builtin command
            let success: bool = built_in.run(input, cmd_line);
            match success {
                true => { println!("Successful builtin command."); }
                false => { println!("Failed builtin command."); }
            }
        } else {
            self.run_custom_cmdline(cmd_line);
        }

        built_in.record_cmd(cmd_line.to_string());
    }

    fn run_custom_cmdline(&self, cmd_line: &str) {
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

fn get_cmdline_from_args() -> Option<String> {
    /* Begin processing program arguments and initiate the parameters. */
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "", "", "");

    opts.parse(&args[1..]).unwrap().opt_str("c")
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();

    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_custom_cmdline(&cmd_line),
        None           => Shell::new("gash > ").run(),
    }
}



// Options to add internal programs
// - add to the match statement
// - add some logic in run_cmdline
// - add some logic in run_cmd
// - (stolen answer) add a struct to handle logic and have builtins vs everything else
//   - command is always the first string
//   - decent way to address issues like: "need more complex logic + options"
//   - if its overkill, can always revert to using a simple function again
// - start with handling one internal command

// Builtin struct
// - it can have a list of &str which are known to be builtins
// - it can redirect to specific cmd
// - define a trait called cmd_type
//   - it will have a run command
//   - it will have a parse_args command

// cmd struct
// - it will have fields which represent options
// - it will implement cmd_run

// - q: should we worry about mixing cmd + normal bash commands?
//   - maybe worry as next step
//   - maybe this is not that big of a deal if the pipe is implemented properly

// Run as background processes
// - research safe ways to run background processes
// - q: why are fork() and execv() bad ideas?

// Pipes
// - split strings via |
// - q: how do you pipe in input from one place to another?

// Signals
// - q: do i have to keep a list of background processes?
// - q: will ctrl-c only kill the proess out front?


