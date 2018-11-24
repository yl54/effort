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

use getopts::Options;
use std::env;

pub mod gash;

fn get_cmdline_from_args() -> Option<String> {
    /* Begin processing program arguments and initiate the parameters. */
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "", "", "");

    opts.parse(&args[1..]).unwrap().opt_str("c")
}

fn main() {
    gash::Shell::new("gash > ").run();
}

/*
Options to add internal programs
- add to the match statement
- add some logic in run_cmdline
- add some logic in run_cmd
- (stolen answer) add a struct to handle logic and have builtins vs everything else
  - command is always the first string
  - decent way to address issues like: "need more complex logic + options"
  - if its overkill, can always revert to using a simple function again
- start with handling one internal command

Builtin struct
- it can have a list of &str which are known to be builtins
- it can redirect to specific cmd
- define a trait called cmd_type
  - it will have a run command
  - it will have a parse_args command

cmd struct
- it will have fields which represent options
- it will implement cmd_run

- q: should we worry about mixing cmd + normal bash commands?
  - maybe worry as next step
  - maybe this is not that big of a deal if the pipe is implemented properly

Run as background processes
- research safe ways to run background processes
- q: why are fork() and execv() bad ideas?
  - fork copies the entire process, hard to ensure safety
- should i spin up a thread?
- is there a safe thread object thingy to spin up?
- q: what is data that may be shared between different threads?
  - history list?
  - current state
- maybe there should be an object that should be launched
  - take a deeper dive into Send and Sync traits
- thread
  - `spawn` a thread
  - guarantee it to finish running by using `join`
    - if the background process is not supposed to finish, then how to stop it?
  - `move` to transfer ownership
    - q: what if there are multiple objects to mv and share in the threads? 
         doesn't that mean there are multiple threads with access to an object?
- use a `mutex` as well
  - acquire the `lock`, which is a smart pointer
- start with making a quick practice program
- thread pool
  - it looks like there is no official thread pool library
  - maybe need to practice by making my own thread pool library
- useful to learn about unsafe rust at some point
- potential solutions
  - just spawn a thread whenever it shows up
    - simple solution
  - keep a threadpool ready
    - harder solution
  - are there other things besides threads?
- probably want to have background processes be async
- tasks
  - pieces of work that run independently
  - does not require an entire os thread
  - The key idea is that, any time a task would block waiting for some external 
    event to occur, it instead returns control to the thread that was executing 
    it (the "executor"), which can make progress on another task instead.
  - goal: make it possible to run any number of tasks cooperatively on a single os thread
  - use the most simple data structures that you can
- q: can you do this without spawning a thread?

Pipes
- split strings via |
- q: how do you pipe in input from one place to another?

Signals
- q: do i have to keep a list of background processes?
- q: will ctrl-c only kill the proess out front?
*/

