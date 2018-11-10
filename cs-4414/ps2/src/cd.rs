// File to handle the `cd` command

use std::fmt::{Display, Formatter, Result};
use std::env;
use std::path::{Path};
use std::string::String;

pub fn run(input: &str) -> bool {
    let spl: Vec<&str> = input.split(" ").collect();

    println!("spl: {:?}", spl);

    // Check the arg parameters and if they are useful.
    let (att_dest, success) = match spl.len() {
        0 => { ("", false) },
        1 => { ("", true) },
        2 => { (spl[1], true) },
        _ => { ("", false) },      
    };

    if !success {
        return false;
    }

    return env::set_current_dir(att_dest).is_ok();
}
