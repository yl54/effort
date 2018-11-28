// This file implements the executor layer of the shell.

use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::vec::Vec;

// Executor struct
pub struct Executor {
    history_list: Vec<String>,

    // TODO: Find a safe way to get this cloned into the thing
    //       The solution is probably an ARC
    pub tx_pipe: Sender<String>,
}

// implementation
impl Executor {
    pub fn new(h_list: Vec<String>, tx: Sender<String>) -> Executor {
    // pub fn new(h_list: Vec<String>) -> Executor {
        Executor { 
            history_list:    h_list,
            tx_pipe: tx,
        }
    }

    // run_cmd runs the built in command.
    pub fn run_cmd(&mut self, argv: Vec<&str>) {
        match argv[0] {
            "cd" => self.cd(&argv[1..]),
            "history" => self.history(),
            _ => { self.run_custom_cmd(argv) }
        }
    }

    // run_custom_cmd runs the custom command passed in.
    fn run_custom_cmd(&mut self, argv: Vec<&str>) {
        // println!(format!("start custom commmand: {:#?}", _args));

        if !self.path_cmd_exists(argv[0]) {
            println!("Custom command does not exist.");
            return;
        }

        match argv.first() {
            Some(&program) => {
                // At this point, its already been confirmed that the command exists on PATH.
                // TODO: Figure out how to get to the send_message
                io::stdout().write(&Command::new(program).args(&argv[1..]).output().unwrap().stdout).unwrap();
            },
            None => (),
        };

        // println!("end custom commmand.");
    }

    // path_cmd_exists checks if the command exists on PATH.
    fn path_cmd_exists(&mut self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }

    fn cd(&mut self, _args: &[&str]) {
        // self.send_message(format!("start cd commmand: {:#?}", _args));
        let (dest, success) = match _args.len() {
            0 => { ("", true) },
            1 => { (_args[0], true) },
            _ => { ("", false) },      
        };

        if !success {
            // self.send_message("cd commmand failed.".to_string());
        }

        let success: bool = env::set_current_dir(dest).is_ok();
        // self.send_message(format!("cd commmand status: {}", success));

        // self.send_message("end cd commmand".to_string());
    }

    fn history(&mut self) {
        self.send_message("start history commmand".to_string());

        // Clone each history value dedicated for letting the println function borrow it.
        // Stolen from fletcher
        let mut results = vec![];
        for history_record in &self.history_list {
            results.push(history_record.clone());
        }
        self.send_message(format!("{:#?}", results));
        println!("{}", format!("{:#?}", results));

        self.send_message("end cd commmand".to_string());
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