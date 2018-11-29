// This file has the implementation of the gash shell.
// This represents the REPL layer of the shell.

use executor::Executor;
use getopts::Options;
use scheduler::Scheduler;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::vec::Vec;

pub struct Shell<'a> {
    cmd_prompt: &'a str,

    history_list: Vec<String>,

    //       This will serve as the messages that are shown on the shell prompt
    //       Q: Why does it need to be wrapped in an Option?
    pub tx_pipe: Sender<String>,
    pub rx_pipe: Option<Box<Receiver<String>>>,

    ex: Executor,

    sc: Scheduler,
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        let tx_ref = tx.clone();
        Shell { 
            cmd_prompt: prompt_str,
            history_list:    vec![],
            tx_pipe: tx,
            rx_pipe: Some(Box::new(rx)),
            ex: Executor::new(tx_ref),
            sc: Scheduler{},
        }
    }

    // run is the main loop for the gash shell.
    pub fn run(&mut self) {
        let mut stdout = io::stdout();
        let count = 0;

        loop {
            // TODO: Figure out how to make this a part of send_message
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            let stdin = io::stdin();

            // Leave a blocking call for input.
            let mut raw_input = String::new();
            stdin.read_line(&mut raw_input).unwrap();
            let input = raw_input.trim();

            // Copy the input and record to history.
            // This provides the shell a dedicated copy that is not shared with commands. 
            // Copied from fletcher
            self.ex.add_to_history(input.to_string());

            self.ex.set_current_cmd(input.to_string());

            // Check if we need to exit
            match input {
                ""      =>  { continue; }
                "exit" => { return; }
                _ => { }
            }

            // Check if it is an asynchronous execution
            // let count = self.history_list.len() - 1;
            let spl: Vec<&str> = input.split(" ").collect();
            match spl[spl.len() - 1] {
                "&" => { 
                    self.sc.run_executor(self.ex.clone()); 
                }          
                _ => { self.ex.run_cmd(); }
            }
        }
    }

    // TODO: Add an async command here
}
