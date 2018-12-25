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

use std::thread;

pub mod gash;
pub mod executor;
pub mod scheduler;

const COMMAND_PROMPT: &str = "gash > ";

fn main() {
    let mut g: gash::Shell = gash::Shell::new(COMMAND_PROMPT);

    // Acquire the receiving end of the printer and print the result
    let rx = g.rx_pipe.take();
    thread::spawn(move || {
        let chan = rx.unwrap();
        loop {
            match chan.recv() {
                Ok(msg) => {
                    println!("{}", msg);
                }
                Err(_e) => {}
            }
        }
    });
    g.run();
}
