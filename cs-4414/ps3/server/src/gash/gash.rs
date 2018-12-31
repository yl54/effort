// This file has the implementation of the gash shell.
// This represents the REPL layer of the shell.

use gash::executor::Executor;
use scheduler::Scheduler;
use std::env;
use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::vec::Vec;

pub struct Shell<'a> {
    cmd_prompt: &'a str,
    history_list: Vec<String>,

    //       This will serve as the messages that are shown on the shell prompt.
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
            sc: Scheduler::new(),
        }
    }

    // run is the main loop for the gash shell.
    pub fn run(&mut self) {
        let mut stdout = io::stdout();

        loop {
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            // Leave a blocking call for input.
            let stdin = io::stdin();
            let mut raw_input = String::new();
            stdin.read_line(&mut raw_input).unwrap();
            let input = raw_input.trim();     

            // Check if we need to exit or continue.
            match input {
                ""      =>  { continue; }
                "exit" => { return; }
                _ => { }
            }

            // Copy the input and record to history.
            // This provides the shell a dedicated copy that is not shared with commands. 
            // Copied from fletcher
            self.add_to_history(input.to_string());
            self.ex.set_current_cmd(input.to_string());
            
            // Check if it's any custom type.
            let argv: Vec<&str> = input.split(" ").collect();
            match argv[0] {
                "cd" => { 
                    self.cd(&argv[1..]);
                    continue;
                }
                "history" => {
                    self.history();
                    continue;
                }
                "plist" => {
                    self.plist();
                    continue;
                }
                _ => {}
            }

            // Check if it is an asynchronous execution.
            match argv[argv.len() - 1] {
                "&" => {
                    self.sc.run_executor(self.ex.clone()); 
                }          
                _ => { self.ex.run_cmd(); }
            }
        }
    }

    // add_to_history adds the input to the history list.
    fn add_to_history(&mut self, input: String) {
        let historical_copy = input.clone();
        self.history_list.push(historical_copy.to_string());
    }

    // cd executes the cd bash command. It is assumed that this will not be used asynchronously. If it needs to, then add it to the executor.
    fn cd(&mut self, _args: &[&str]) {
        // println!(format!("start cd commmand: {:#?}", _args));
        let (dest, success) = match _args.len() {
            0 => { ("", true) },
            1 => { (_args[0], true) },
            _ => { ("", false) },      
        };

        if !success {
            // println!("cd commmand failed.".to_string());
        }

        // let success: bool = env::set_current_dir(dest).is_ok();
        env::set_current_dir(dest).is_ok();
        // println!(format!("cd commmand status: {}", success));

        // println!("end cd commmand".to_string());
    }

    // history executes the history bash command. It is assumed that this will not be used asynchronously. If it needs to, then add this and the history_list to the executor.
    fn history(&mut self) {
        // println!("{}", "start history commmand".to_string());

        // Clone each history value dedicated for letting the println function borrow it.
        // Stolen from fletcher
        let mut results = vec![];
        for history_record in &self.history_list {
            results.push(history_record.clone());
        }
        println!("{}", format!("{:#?}", results));

        // println!("{}", "end history commmand".to_string());
    }

    // plist lists the processes recorded by the scheduler.
    fn plist(&mut self) {
        self.sc.show_process_list();
    }
}
