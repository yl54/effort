// This file contains a webserver.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Mutex, Arc};

use server::handlers;
use server::utils;

// Server address
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

// Tip: For complex things like function parameters, denote with a type.
type Callback = fn(&mut TcpStream);

struct Handler {
    // The path of the handler
    path: String,

    // The function for the handler
    handler: Callback,

    // The number of times this handler has been requested
    count: Arc<Mutex<u16>>,
}

// Webserver struct
pub struct Webserver {
    // Handle to the tcp listener
    listener: TcpListener,

    // Hashmap for handler functions. The string path to handler struct    
    handlers: HashMap<String, Handler>,

    // Count of how many requests total requests have been served
    requests_total: Arc<Mutex<u16>>,
}

// Webserver implementation
impl Webserver {
    // New function
    pub fn new() -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        // Tcp listeners are the default thing to listen to requests.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");       

        // Return the Webserver. 
        return Webserver {
            listener: listener,
            handlers: HashMap::new(),
            requests_total: Arc::new(Mutex::new(0)),
        };
    }

    // register_handler registers a handler with a path.
    pub fn register_handler(&mut self, path: String, handler: Callback) {
        let cl_path = path.clone();

        let h = Handler {
            path: path,
            handler: handler,
            count: Arc::new(Mutex::new(0)),
        };

        // Add the path + handler function to the hashmap
        self.handlers.insert(cl_path, h);
    }

    // Listen for requests and handle requests.
    pub fn listen(&mut self) {
        // Start the listener infinite loop.
        for stream in self.listener.incoming() {
            // Increase the value of the int the reference is pointing to.
            let mut num_total = self.requests_total.lock().unwrap();
            *num_total += 1;

            match stream {
                // Handle if an error.
                Err(_) => (),

                // Handle if its a Result object.
                Ok(mut stream) => {
                    // Read from the stream.
                    // Q: Why do you have to read from the stream before stuff is written into it?
                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();

                    // Extract the body from the stream.
                    let body: &str = match str::from_utf8(&buf) {
                        Err(error) => {
                            println!("Received request error:\n{}", error);
                            return;
                        },
                        Ok(body) => body,
                    };
                    
                    // Extract the path from the body.
                    let path = utils::extract_path(&body).to_string();
                    match self.handlers.get(&path) {
                        Some(h) => { 
                            (h.handler)(&mut stream);
                            let mut c = h.count.lock().unwrap();
                            *c += 1;
                        },
                        None => {
                            handlers::handle_default(&mut stream); 
                        },
                    }
                }
            }
        }
    }
}