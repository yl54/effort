// This file implements the executor layer of the shell.

use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::vec::Vec;

// Executor struct
#[derive(Clone, Debug)]
pub struct Executor {
    current_cmd: String,
    tx_pipe: Sender<String>,
}

// implementation
impl Executor {
    pub fn new(tx: Sender<String>) -> Executor {
        Executor {
            current_cmd: "".to_string(),
            tx_pipe: tx,
        }
    }

    // set_current_cmd sets the command to run for the executor
    pub fn set_current_cmd(&mut self, input: String) {
        self.current_cmd = input.clone();
    }

    // run_cmd runs the built in command.
    pub fn run_cmd(&mut self) {
        let cl = self.current_cmd.clone();
        let argv: Vec<&str> = cl.split(" ").collect();

        // println!(format!("start custom commmand: {:#?}", _args));
        if !self.path_cmd_exists(argv[0]) {
            println!("Custom command does not exist.");
            return;
        }

        match argv.first() {
            Some(&program) => {
                let result = &Command::new(program).args(&argv[1..]).output().unwrap().stdout;
                let string_result = str::from_utf8(&result).unwrap().to_string();
                println!("{}", string_result.to_string());
                self.send_message(string_result);
            },
            None => {
                self.send_message("No command existed".to_string());
            },
        };

        // println!("end custom commmand.");
    }

    // path_cmd_exists checks if the command exists on PATH.
    fn path_cmd_exists(&mut self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }

    // send_message takes any message and sends it to a queue to display as output
    fn send_message(&mut self, msg: String) {
        // Check if the send into the pipe worked
        match self.tx_pipe.send(msg) {
            Ok(_) => {}
            Err(_e) => {}
        };
    }
}