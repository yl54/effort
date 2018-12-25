// This file implements the executor layer of the shell.

use getopts::Options;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::vec::Vec;


// Piping delimiters
const INPUT_CH: char = '<';
const OUTPUT_CH: char = '>';
const PIPE_CH: char = '|';

// Delimiter for stopping
const SEMICOLON_CH: char = ';';

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
        
        // Do a for loop for input/output

        // Maybe it should have each its own input and output
        // https://doc.rust-lang.org/std/process/struct.Command.html#method.stdin


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
            }
        };

        // println!("end custom commmand.");
    }

    pub fn run_cmd_loop(&mut self) {
        // Split up the command into piped pieces
        let (cmd_parts, pipe, ok) = self.split_pipe();

        // Check if the split worked.
        if !ok {
            return;
        }

        // Record length of the cmd and pipe
        let cmd_len = cmd_parts.len();
        let pipe_len = pipe.len();

        // Check if there is only one command that exists
        if cmd_len == 1 {
            self.run_single_cmd(cmd_parts[0].clone());
        }

        // Keep track of which index of cmd part its at
        let mut cmd_index = 0;

        // Keep track of pipe index
        let mut pipe_index = 0;

        // Keep track if things are still being piped
        let piped = false;

        // Keep track of output for piping
        let mut output: String = "".to_string();

        // Keep track if its a piped request

        // Use a while loop over the index of the cmd part.
        // Check if the index is less than length
        while cmd_index < cmd_len - 1 {
            let cmd_cl = cmd_parts[cmd_index].clone();

            // Check if there is anything in the pipe
            if cmd_index >= pipe_len {
                self.run_single_cmd(cmd_cl);
                return;
            }

            // split up the command into pieces
            let argv: Vec<&str> = cmd_cl.split(" ").collect();
            let cl_arg_0 = argv[0].to_string().clone();
            let cl_argv = argv.clone();

            // Check if the command exists. This string at this index should always be on a command, never file
            // NOTE: f.txt | grep is not a thing
            if !self.path_cmd_exists(cl_arg_0.as_str()) {
                let message = format!("Nothing exists for this command: {}", cl_arg_0);
                self.send_message(message);
                return;
            }

            // Keep track of the input.
            // Multiple input characters means concactenate to the string for input
            // Copy the output variable on the outside, and continue to append to that 
            // `cat Cargo.toml | grep name < Cargo.lock` is valid and just appends one to the other
            let mut input: String = output.clone();

            // Keep track of the output file.
            // keep track if an output file exists.
            // Only write to the last one
            // NOTE: There may be no output file.
            let mut is_output_file = false;
            let mut output_file = String::new();

            // Keep track if a pipe has been seen.
            // If there is a pipe, then stop while loop. Handle it in the next one.
            let mut is_output_pipe = false;

            // Do a while loop over the pipe vec
            while cmd_index < cmd_len - 1 && !is_output_pipe {
                cmd_index += 1;
                let current_cmd = cmd_parts[cmd_index].clone();
                let current_ch = pipe[pipe_index];

                // If there is input, then append to the standard input
                if current_ch == INPUT_CH {
                    let current_path = Path::new(current_cmd.as_str());
                    let display = current_path.display();

                    // check if the input at cmd_index is a file that exists
                    if !current_path.is_file() {
                        let message = format!("provided input is not a file: {}", display);
                        self.send_message(message);
                        return;
                    }

                    // Open the path in read-only mode, returns `io::Result<File>`
                    let mut file = match File::open(&current_path) {
                        Err(why) => {
                                        let message = format!("couldn't open {}: {}", display,
                                                                   why.description());
                                        self.send_message(message);
                                        return;
                                    },
                        Ok(file) => file,
                    };

                    // Read the file contents into a string, returns `io::Result<usize>`
                    let mut s = String::new();
                    match file.read_to_string(&mut s) {
                        Err(why) => {
                                        let message = format!("couldn't read {}: {}", display,
                                                                   why.description());
                                        self.send_message(message);
                                        return;
                                    },
                        Ok(_) => print!("{} contains:\n{}", display, s),
                    }

                    // Concactenate input to already existing input
                    input = format!("{}\n{}", input, s);
                
                // If there is an output, then check if the next one is a file
                } else if current_ch == OUTPUT_CH {
                    // check if the output at cmd_index is a valid file
                    // it basically has to be a string with no whitespace
                    let arg: Vec<&str> = current_cmd.split(" ").collect();
                    if arg.len() > 1 {
                        let message = format!("output file is invalid: {}", current_cmd);
                        self.send_message(message);
                        return;
                    }

                    // if it is valid, set the output file and bool
                    is_output_file = true;
                    output_file = current_cmd.clone();

                // If there is an pipe, then end and indicate there is a pipe
                // Exit loop no matter what
                } else if current_ch == PIPE_CH {
                    // Check if an output command already exists
                    // if not, then mark it as done b/c pipe
                    if !is_output_file {
                        is_output_pipe = true;
                    }
                    
                    break;
                }

                pipe_index += 1;
            }

            // Spawn a command
            let mut process = match Command::new(cl_arg_0)
                                        .stdin(Stdio::piped())
                                        .stdout(Stdio::piped())
                                        .args(&cl_argv[1..])
                                        .spawn() {
                Err(why) => {
                                let message = format!("couldn't spawn wc: {}", why.description());
                                println!("{}", message);
                                return;
                            },
                Ok(process) => process,
            };


            // Handle the input for the process.
            // Q: why does this need to be in its own scope
            {
                // The `stdin` field has type `Option<PipeStream>`
                // `take` will take the value out of an `Option`, leaving `None` in
                // its place.
                //
                // Note that we take ownership of `stdin` here
                let mut stdin = process.stdin.as_mut().unwrap();
                let cl_input = input.clone();

                // Write a string to the stdin of the command
                // match stdin.write_all(b"PANGRAM") {
                let input_bytes = input.into_bytes();
                match stdin.write_all(&input_bytes) {
                    Err(why) => panic!("couldn't write to process stdin: {}", "why.desc"),
                    Ok(_) => println!("wrote to stdin: {}", cl_input),
                }

                // `stdin` gets `drop`ed here, and the pipe is closed
                // This is very important, otherwise `wc` wouldn't start processing the
                // input we just sent
                println!("made it here 1:");
            }

            // TODO: Fails here if there is input
            

            // Execute the command
            let mut stdout_str = String::new();
            println!("made it here 2:");
            
            let current_output = match process.wait_with_output() {
                Err(why) => {
                                let message = format!("couldn't read commands stdout:{}",
                                                                why.description());
                                println!("{}", message);
                                stdout_str.clone()
                            },
                Ok(thing) => String::from_utf8(thing.stdout).expect("Not UTF-8")
            };

            println!("current_output: {}", current_output);

            output = current_output;

            println!("made it here 3:");

            // Check why the loop ended
            // If there was an output file, then write to that file.
            if is_output_file {
                let output_path = Path::new(output_file.as_str());
                let display = output_path.display();

                // Open a file in write-only mode, returns `io::Result<File>`
                let mut file = match File::create(&output_path) {
                    Err(why) => {
                                    let message = format!("couldn't create {}: {}",
                                                               display,
                                                               why.description());
                                    self.send_message(message);
                                    return;
                                },
                    Ok(file) => file,
                };

                // Write the output string to `file`, returns `io::Result<()>`
                match file.write_all(stdout_str.as_bytes()) {
                    Err(why) => {
                        let message = format!("couldn't write to {}: {}",
                                                               display,
                                                               why.description());
                        self.send_message(message);
                        return;
                    },
                    Ok(_) => println!("successfully wrote to {}", display),
                }

                // Flush out the output
                break;

            // If there was a pipe found, then let the outside output be what the output is here
            } else if is_output_pipe {
                output = stdout_str;
            }            
        }

        // If there is remaining output, then send the message out
        self.send_message(output);
    }

    fn run_single_cmd(&mut self, cmd: String) {
        // Get the program 
        let cl = cmd.clone();
        let cl_trim = cl.trim().to_string();
        let argv: Vec<&str> = cl_trim.split(" ").collect();

        // Check if its empty
        if argv.len() == 0 {
            self.send_message("No command existed".to_string());
            return;
        }

        if !self.path_cmd_exists(argv[0]) {
            println!("Custom command does not exist.");
            return;
        }

        // Clone the command again to provide a copy to pass in
        let cl_argv = argv.clone();
        let cl_arg_0 = argv[0].to_string().clone();

        // Create the command struct
        let mut exec = Command::new(cl_arg_0);
        let exec_args = exec.args(&cl_argv[1..]);

        // Pass in the input into the command struct

        // Get the stdout of the command struct
        let output = exec_args.output().unwrap().stdout;
        let output_str = str::from_utf8(&output).unwrap();

        // Set the stdin as the stdout
        self.send_message(output_str.to_string());
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

    // get_cmd returns the command loaded into the Executor.
    pub fn get_cmd(&mut self) -> String {
        let s = self.current_cmd.clone();
        return s;
    }

    // split_pipe examines the full command and splits it into separate pieces for piping purposes
    // Currently, it only handles the happy path of stuff
    // Its not that robust.
    // It returns the command parts, piping directions, and if everything was successful or not. 
    fn split_pipe(&mut self) -> (Vec<String>, Vec<char>, bool) {
        // Get a copy of the string input
        let str_cpy = self.current_cmd.clone();

        // Define the piping characters
        let piping_regex = &[INPUT_CH, OUTPUT_CH, PIPE_CH];

        // Read each character individually for the piping characters
        // It doesn't handle the case if a piping character is in the quotes
        let mut piping_vec = vec![];
        for c in str_cpy.chars() {
            if piping_regex.contains(&c) {
                piping_vec.push(c);
            }
        }

        // Split up the full command by the piping characters
        let mut cmd_parts = vec![];
        for part in str_cpy.split(&piping_regex[..]) {
            cmd_parts.push(part.to_string().trim().to_string());
        }

        // Validate there are proper piping directions.
        if piping_vec.len() + 1 != cmd_parts.len() {
            // This is a failure. Return a failed thing.
            return (vec![], vec![], false) 
        }

        println!("{:?}", cmd_parts.clone());
        println!("{:?}", piping_vec.clone());

        return (cmd_parts, piping_vec, true)
    }
}