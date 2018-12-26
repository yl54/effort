// This file contains a webserver.

use std::net::{TcpListener, TcpStream};

// File to return to users

// Server address
const SERVER_ADDR: &str = "";

// Webserver struct
pub struct Webserver {
    // Count of how many requests it has served

    // Handle to the tcp listener
    listener: TcpListener,

    // Hashmap for handler functions. String to function handler
}

// Webserver implementation
impl Webserver {
    // New function
    pub fn new() -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        // Tcp listeners are the default thing to listen to requests.
        let listener = TcpListener::bind(SERVER_ADDR).expect("Could not bind to address.");        

        // Return the Webserver. 
        return Webserver{
            listener: listener,
        };
    }

    // Listen for requests and simply handle requests
    pub fn listen(&self) {
        // Start the listener infinite loop
        for stream in self.listener.incoming() {
            // Return a webpage with the request
        }
    }
}
