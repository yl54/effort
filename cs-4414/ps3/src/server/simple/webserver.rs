// This file contains a webserver.

use std::error::Error;
// use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::sync::{Arc, Mutex};

use server::simple::scheduler::{Callback, Scheduler};

// Server address
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

// Webserver 
pub struct Webserver {
    // l is the handle to the request listener.
    l: TcpListener,

    // sc is a scheduler for each connection that comes to the Webserver.    
    sc: Scheduler,

    // req_total is the count of how many total requests have been recieved.
    req_total: Arc<Mutex<u16>>,
}

impl Webserver {
    // New function
    pub fn new() -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");

        return Webserver {
            l: listener,
            sc: Scheduler::new(),
            req_total: Arc::new(Mutex::new(0)),
        };
    }

    // register_handler registers a handler with a path.
    pub fn register_handler(&mut self, path: String, handler: Callback) {
        self.sc.register_handler(path, handler);
    }

    // listen listens for requests and executes the proper handler if possible.
    pub fn listen(&mut self) {
        // Start the listener infinite loop.
        for stream in self.l.incoming() {
            // Increase the total request count.
            let mut num_total = self.req_total.lock().unwrap();
            *num_total += 1;

            match stream {
                Err(err) => debug!("Couldn't read the stream: {}", err.description()),
                Ok(mut stream) => {
                    self.sc.schedule_stream(stream);
                }
            }
        }
    }
}
