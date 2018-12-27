// This file contains a webserver.

use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// File to return to users

// Server address
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

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
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");       

        // Return the Webserver. 
        return Webserver{
            listener: listener,
        };
    }

    // Listen for requests and simply handle requests
    pub fn listen(&self) {
        // Start the listener infinite loop
        for stream in self.listener.incoming() {
            match stream {
                // Handle if an error
                Err(_) => (),

                // Handle if its a Result object
                Ok(mut stream) => {
                    // Read from the stream.
                    // Q: Why do you have to read from the stream before stuff is written into it?
                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();

                    // Return a webpage for the request
                    self.handle_request(&mut stream);
                }
            }
        }
    }

    // handle_request reads the path and gives a response.
    fn handle_request(&self, stream: &mut TcpStream) {
        // Pick the page based off of the request.
        let html_file_path: &str = "files/original.html";

        // Create the full http response based off of the page
        let html_file_contents: String = self.get_response_page(html_file_path);

        // Return the string of the http response
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", html_file_contents);
        stream.write(response.as_bytes()).unwrap();
    }

    // get_response_page attempts to get the contents of the page.
    fn get_response_page(&self, file_path: &str) -> String {
        // Attempt to open the file.
        let mut f = File::open(file_path).expect("file not found");

        // Allocate a new string, which is dynamic.
        let mut contents = String::new();

        // Read the file into the string
        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        // Return the string object.
        return contents;
    }
}
