// This file contains a webserver.

use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::handlers;
use server::utils;

const NUM_OF_HEADERS: usize = 30;

// Server address
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

// Tip: For complex things like function parameters, denote with a type.
type Callback = fn(&mut TcpStream);

struct Handler {
    // path is the url path.
    path: String,

    // handler is the function to execute.
    handler: Callback,

    // count is the number of times this handler has been requested.
    count: Arc<Mutex<u16>>,
}

// Webserver 
pub struct Webserver {
    // tcp_listener is the handle to the request listener.
    listener: TcpListener,

    // handlers is a hashmap from the url path to handler functions.    
    handlers: HashMap<String, Handler>,

    // requests_total is the count of how many total requests have been recieved.
    requests_total: Arc<Mutex<u16>>,
}

impl Webserver {
    // New function
    pub fn new() -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");       

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

        // Add the path + handler combination.
        self.handlers.insert(cl_path, h);
    }

    // listen listens for requests and executes the proper handler if possible.
    pub fn listen(&mut self) {
        // Start the listener infinite loop.
        for stream in self.listener.incoming() {
            // Increase the value of the int the reference is pointing to.
            let mut num_total = self.requests_total.lock().unwrap();
            *num_total += 1;

            match stream {
                // Handle if an error.
                Err(err) => debug!("Couldn't read the stream: {}", err.description()),

                // Handle if its a Result object.
                Ok(mut stream) => {
                    // Read from the stream.
                    // Q: Why do you have to read from the stream before stuff is written into it?
                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();

                    // Extract the body and path from the stream.
                    let mut headers = [EMPTY_HEADER; NUM_OF_HEADERS];
                    let mut req = Request::new(&mut headers[..]);
                    let status = match req.parse(buf.as_ref()) {
                        Ok(s) => s,
                        Err(err) => {
                            debug!("Failed parsing the bytes into a request.");
                            continue
                        },
                    };

                    let path = match req.path {
                        Some(p) => p,
                        None => {
                            debug!("Failed getting the path from the request.");
                            continue
                        },
                    };

                    match self.handlers.get(&path.to_string()) {
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
