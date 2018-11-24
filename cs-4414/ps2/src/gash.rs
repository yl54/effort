// This file has the implementation of the gash shell.

use getopts::Options;
use std::env;
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
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
    //       Q: Why does it need to be wrapped in an Option?
    pub tx_pipe: Option<Box<Sender<String>>>,
    pub rx_pipe: Option<Box<Receiver<String>>>,
}

impl <'a>Shell<'a> {
    pub fn new(prompt_str: &'a str) -> Shell<'a> {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        Shell { 
            cmd_prompt: prompt_str,
            history_list:    vec![],
            tx_pipe: Some(Box::new(tx)),
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
    fn is_built_in(&mut self, input: &str) -> bool {
        // return the result of contains
        return self.contains(input);
    }

    // contains checks the built in list to see if it is a built in command.
    fn contains(&mut self, input: &str) -> bool {
        for i in 0..CMD_LIST.len() {
            if input == CMD_LIST[i] {
                return true;
            }
        }

        return false;
    }    

    // TODO: Add builtin stuff here
    // run runs the built in command.
    fn run_cmd(&mut self, argv: Vec<&str>) {
        match argv[0] {
            "cd" => self.cd(&argv[1..]),
            "history" => self.history(),
            _ => { self.run_custom_cmd(argv) }
        }
    }

    // run_custom_cmd runs the custom command passed in.
    fn run_custom_cmd(&mut self, argv: Vec<&str>) {
        // self.send_message(format!("start custom commmand: {:#?}", _args));

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

        // self.send_message("end custom commmand.");
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
            self.send_message("cd commmand failed.".to_string());
        }

        let success: bool = env::set_current_dir(dest).is_ok();
        self.send_message(format!("cd commmand status: {}", success));

        // self.send_message("end cd commmand".to_string());
    }

    fn history(&mut self) {
        // self.send_message("start history commmand".to_string());

        // Clone each history value dedicated for letting the println function borrow it.
        // Stolen from fletcher
        let mut results = vec![];
        for history_record in &self.history_list {
            results.push(history_record.clone());
        }
        self.send_message(format!("{:#?}", results));

        // self.send_message("end cd commmand".to_string());
    }

    // send_message takes any message and sends it to a queue to display as output
    fn send_message(&mut self, msg: String) {
        // Check the sending pipe
        match &self.tx_pipe {
            // If it exists, then attempt to send to pipe
            // q: what are scenarios where this will not exist?
            Some(pipe) => {
                // Check if the send into the pipe worked
                match pipe.send(msg) {
                    Ok(_) => {}
                    Err(_e) => {}
                };
            }

            // If it doesnt exist, then do nothing
            None => {}
        }
    }

    // TODO: Add an async command here
}
