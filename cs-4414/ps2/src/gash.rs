// This file has the implementation of the gash shell.
// This represents the REPL layer of the shell.

use executor;
use getopts::Options;
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
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        Shell { 
            cmd_prompt: prompt_str,
            history_list:    vec![],
            tx_pipe: tx,
            rx_pipe: Some(Box::new(rx)),
        }
    }

    // run is the main loop for the gash shell.
    pub fn run(&mut self) {
        let mut stdout = io::stdout();

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
            let historical_copy = input.clone();
            self.history_list.push(historical_copy.to_string());

            // Check if we need to exit
            match input {
                ""      =>  { continue; }
                "exit" => { return; }
                _ => { }
            }

            // Split the string by " "
            let spl: Vec<&str> = input.split(" ").collect();

            // Create a new Executor object
            // NOTE: Copying the history list multiple times will suck if
            //       there are many records. 
            let mut results = vec![];
            for history_record in &self.history_list {
                results.push(history_record.clone());
            }

            // Clone the sender
            let tx_ref = self.tx_pipe.clone();

            let mut ex = executor::Executor::new(results, tx_ref);

            // Check if it is an asynchronous execution
            match spl[spl.len() - 1] {
                "&" => { ex.run_cmd(spl); }
                _ => { ex.run_cmd(spl); }
            }
        }
    }

    // TODO: Add an async command here
}
