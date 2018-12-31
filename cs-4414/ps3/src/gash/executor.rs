// This file implements the executor layer of the shell.

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::sync::mpsc::Sender;
use std::vec::Vec;

// Piping delimiters
const INPUT_CH: char = '<';
const OUTPUT_CH: char = '>';
const PIPE_CH: char = '|';

// Executor struct
#[derive(Clone, Debug)]
pub struct Executor {
    current_cmd: String,
    tx_pipe: Sender<String>,
}

// An Executor takes a string input and executes the shell command.
impl Executor {
    pub fn new(tx: Sender<String>) -> Executor {
        Executor {
            current_cmd: "".to_string(),
            tx_pipe: tx,
        }
    }

    // set_current_cmd sets the command to run for the executor.
    pub fn set_current_cmd(&mut self, input: String) {
        self.current_cmd = input.clone();
    }

    // run_cmd gets the command input and executes it. 
    // It returns either an error message or the output of the program.
    pub fn run_cmd(&mut self) -> String {
        // Split up the command into piped pieces.
        let (cmd_parts, pipe, ok) = self.split_pipe();
        if !ok {
            let msg = format!("Parsing the command failed.");
            let cl_msg = msg.clone();
            self.send_message(msg);
            return cl_msg;
        }

        // Record length of the cmd and pipe.
        let cmd_len = cmd_parts.len();

        // Check if there is only one command that exists.
        if cmd_len == 1 {
            self.run_single_cmd(cmd_parts[0].clone());
        }

        // Keep track of which index of cmd part its at.
        let mut cmd_index = 0;

        // Keep track of pipe index.
        let mut pipe_index = 0;

        // Keep track of output contents for piping.
        let mut output: String = "".to_string();

        // Keep track if its a piped request.
        let mut is_output_pipe = false; 

        // Use a while loop over the index of the cmd part.
        while cmd_index < cmd_len {
            let cmd_cl = cmd_parts[cmd_index].clone();
            let argv: Vec<&str> = cmd_cl.split(" ").collect();
            let cl_arg_0 = argv[0].to_string().clone();
            let cl_argv = argv.clone();

            // Check if the command exists. This string at this index should always be on a command, never file.
            if !self.path_cmd_exists(cl_arg_0.as_str()) {
                let msg = format!("Nothing exists for this command: {}", cl_arg_0);
                let cl_msg = msg.clone();
                self.send_message(msg);
                return cl_msg;
            }

            // Keep track of the input. Initialize it to the previous output.
            let mut input: String = output.clone();

            // Keep track if the output file exists and its name if it exists
            let mut is_output_file = false;
            let mut output_file = String::new();

            cmd_index += 1;

            // Loop for input and output redirecting commands.
            while cmd_index < cmd_len && !is_output_pipe {
                let current_cmd = cmd_parts[cmd_index].clone();
                let current_ch = pipe[pipe_index];

                // Check if there is an input redirection.
                if current_ch == INPUT_CH {
                    let current_path = Path::new(current_cmd.as_str());
                    let display = current_path.display();

                    // Check if the path is a file that exists.
                    if !current_path.is_file() {
                        let msg = format!("provided input is not a file: {}", display);
                        let cl_msg = msg.clone();
                        self.send_message(msg);
                        return cl_msg;
                    }

                    // Open the path in read-only mode, returns `io::Result<File>`.
                    let mut file = match File::open(&current_path) {
                        Err(why) => {
                                        let msg = format!("couldn't open {}: {}", display,
                                                                   why.description());
                                        let cl_msg = msg.clone();
                                        self.send_message(msg);
                                        return cl_msg;
                                    },
                        Ok(file) => file,
                    };

                    // Read the file contents into a string, returns `io::Result<usize>`.
                    let mut s = String::new();
                    match file.read_to_string(&mut s) {
                        Err(why) => {
                                        let msg = format!("couldn't read {}: {}", display,
                                                                   why.description());
                                        let cl_msg = msg.clone();
                                        self.send_message(msg);
                                        return cl_msg;
                                    },
                        Ok(_) => print!("{} contains:\n{}", display, s),
                    }

                    // Concactenate input to already existing input.
                    input = format!("{}\n{}", input, s);
                
                // Check if there is an output redirection.
                } else if current_ch == OUTPUT_CH {
                    // Check if the command is a valid file.
                    // It has to be a string with no whitespace.
                    let arg: Vec<&str> = current_cmd.split(" ").collect();
                    if arg.len() > 1 {
                        let msg = format!("output file is invalid: {}", current_cmd);
                        let cl_msg = msg.clone();
                        self.send_message(msg);
                        return cl_msg;
                    }

                    is_output_file = true;
                    output_file = current_cmd.clone();

                // Check if there is an input redirection.
                } else if current_ch == PIPE_CH {
                    // Make sure there is no output file.
                    if !is_output_file {
                        is_output_pipe = true;
                    }
                    
                    // Exit loop.
                    break;
                }

                cmd_index += 1;
                pipe_index += 1;
            }

            // Spawn a command.
            let mut process = match Command::new(cl_arg_0)
                                            .stdin(Stdio::piped())
                                            .stdout(Stdio::piped())
                                            .args(&cl_argv[1..])
                                            .spawn() {
                Err(why) => {
                                let msg = format!("couldn't spawn wc: {}", why.description());
                                let cl_msg = msg.clone();
                                self.send_message(msg);
                                return cl_msg;
                            },
                Ok(process) => process,
            };

            // Handle the input for the process.
            {
                // The `stdin` field has type `Option<PipeStream>`
                // `take` will take the value out of an `Option`, leaving `None` in
                // its place.
                //
                // Note that we take ownership of `stdin` here.
                let stdin = process.stdin.as_mut().unwrap();
                let cl_input = input.clone();

                // Write a string to the stdin of the command.
                let cl_input_bytes = input.into_bytes();
                match stdin.write_all(&cl_input_bytes) {
                    Err(why) => panic!("couldn't write to process stdin: {}", why.description()),
                    Ok(_) => println!("wrote to stdin: {}", cl_input),
                }

                // `stdin` gets `drop`ed here, and the pipe is closed.
                // This is very important, otherwise the program wouldn't 
                // start processing the input we just sent.
            }

            // Execute the command.
            let current_output = match process.wait_with_output() {
                Err(why) => {
                                let msg = format!("couldn't read commands stdout:{}",
                                                                why.description());
                                let cl_msg = msg.clone();
                                self.send_message(msg);
                                return cl_msg;
                            },
                Ok(thing) => String::from_utf8(thing.stdout).expect("Not UTF-8")
            };

            // Check if there is an output file. Only write to the last file.
            if is_output_file {
                let output_path = Path::new(output_file.as_str());
                let display = output_path.display();

                // Open a file in write-only mode, returns `io::Result<File>`.
                let mut file = match File::create(&output_path) {
                    Err(why) => {
                                    let msg = format!("couldn't create {}: {}",
                                                               display,
                                                               why.description());
                                    let cl_msg = msg.clone();
                                    self.send_message(msg);
                                    return cl_msg;
                                },
                    Ok(file) => file,
                };

                // Write the output string to `file`, returns `io::Result<()>`.
                file.write_all(current_output.clone().as_bytes()).expect("Unable to write data");
                output = "".to_string();

                // Flush out the output.
                break;

            // Check if there is a pipe output.
            } else if is_output_pipe {
                output = current_output;
            }            
        }

        // If there is remaining output, then send the message out.
        let cl_output = output.clone();
        self.send_message(output);
        return cl_output;
    }

    // run_single_cmd takes a single string input and runs it as one command.
    fn run_single_cmd(&mut self, cmd: String) {
        // Get the program.
        let cl = cmd.clone();
        let cl_trim = cl.trim().to_string();
        let argv: Vec<&str> = cl_trim.split(" ").collect();

        // Check if its empty.
        if argv.len() == 0 {
            self.send_message("No command existed".to_string());
            return;
        }

        if !self.path_cmd_exists(argv[0]) {
            self.send_message("Custom command does not exist.".to_string());
            return;
        }

        // Clone the command again to provide a copy to pass in.
        let cl_argv = argv.clone();
        let cl_arg_0 = argv[0].to_string().clone();

        // Create the command struct.
        let mut exec = Command::new(cl_arg_0);
        let exec_args = exec.args(&cl_argv[1..]);

        // Get the stdout of the command struct.
        let output = exec_args.output().unwrap().stdout;
        let output_str = str::from_utf8(&output).unwrap();

        // Set the stdin as the stdout.
        self.send_message(output_str.to_string());
    }

    // path_cmd_exists checks if the command exists on PATH.
    fn path_cmd_exists(&mut self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }

    // send_message takes any message and sends it to a queue to display as output.
    fn send_message(&mut self, msg: String) {
        // Check if the send into the pipe worked.
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

    // split_pipe examines the full command and splits it into separate pieces for piping purposes.
    // Currently, it only handles the happy path of stuff.
    // Its not that robust.
    // It returns the command parts, piping directions, and the success. 
    fn split_pipe(&mut self) -> (Vec<String>, Vec<char>, bool) {
        // Get a copy of the string input.
        let str_cpy = self.current_cmd.clone();

        // Define the piping characters.
        let piping_regex = &[INPUT_CH, OUTPUT_CH, PIPE_CH];

        // Read each character individually for the piping characters.
        // It doesn't handle the case if a piping character is in the quotes.
        let mut piping_vec = vec![];
        for c in str_cpy.chars() {
            if piping_regex.contains(&c) {
                piping_vec.push(c);
            }
        }

        // Split up the full command by the piping characters.
        let mut cmd_parts = vec![];
        for part in str_cpy.split(&piping_regex[..]) {
            cmd_parts.push(part.to_string().trim().to_string());
        }

        // Validate there are proper piping directions.
        if piping_vec.len() + 1 != cmd_parts.len() {
            // This is a failure. Return a failed thing.
            return (vec![], vec![], false) 
        }

        return (cmd_parts, piping_vec, true)
    }
}