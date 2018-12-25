// This file will be a background process to test out on.

use std::{thread, time};

// main 
fn main() {
    let mut count = 0;
    let ten_seconds = time::Duration::from_secs(10);
    loop {
        println!("Count: {}", count);
        count = count + 1;
        thread::sleep(ten_seconds);
    }
}