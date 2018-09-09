//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

use std::fs::File;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::str;
use std::sync::{Mutex, Arc};
use std::thread;
use std::vec::Vec;

// Function to extract the path of the request
fn extract_path(request: &str) -> &str {
    // Split the string by new line
    let lines: Vec<&str> = request.split("\n").collect();

    // Check if the vec is valid.
    if lines.len() <= 0 {
        return "";
    }

    // Take the first string
    let line: &str = lines[0];

    // Split the string by " "
    let spl: Vec<&str> = line.split(" ").collect();
    println!("{:?}", spl);

    // Check if the vec is valid.
    if spl.len() <= 0 {
        return "";
    }

    // Check the length
    if spl.len() != 3 {
        println!("part does not match format");
        return "";
    }

    // Get the second string
    // take the &str in the middle, ignore the first character
    let part: &str = &spl[1][1..];
    println!("part: {}", part);

    // Try to match and return.
    return part;
}

/*
// Function to handle a request.
fn handle_request(path: &str) -> String {
    // Pick the page based off of the request.
    let html_file_path: &str = select_response_page(path);

    // Create the full http response based off of the page
    let html_file_contents: String = get_response_page(html_file_path);

    // Return the string of the http response
    return format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", html_file_contents);
}
*/

// Function to handle a request.
// Pass in the stream.
fn handle_request(path: &str, stream: &mut TcpStream) {
    // Pick the page based off of the request.
    let html_file_path: &str = select_response_page(path);
    if html_file_path == "" {
        println!("Invalid path: {}", path);
        return;
    }

    // Create the full http response based off of the page
    let html_file_contents: String = get_response_page(html_file_path);

    // Return the string of the http response
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", html_file_contents);
    stream.write(response.as_bytes()).unwrap();
}

// Function to pick a page.
fn select_response_page(path: &str) -> &str {
    // Do a switch statement.
    match path {
        // `original` case
        "original" => "files/original.html",
        // `great` case
        "great" => "files/great.html",
        // `trash` case
        "trash" => "files/trash.html",
        // default case
        _ => "",
    }
}

// Function to get the response page html.
fn get_response_page(file_path: &str) -> String {
    // Attempt to open the file.
    let mut f = File::open(file_path).expect("file not found");

    // Allocate a new string, which is dynamic.
    let mut contents = String::new();

    // Read the file into the string
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Return the string object.
    return contents;
}

fn main() {
    // Make an ipv4 address
    let ipv4_addr: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16  = 4414;
    let ip_addr: IpAddr = IpAddr::V4(ipv4_addr);
    let socket: SocketAddr = SocketAddr::new(ip_addr, port); 

    // let addr = "127.0.0.1:4414";

    // let listener = TcpListener::bind(addr).unwrap();
    let listener = TcpListener::bind(socket).unwrap();

    let count = Arc::new(Mutex::new(0));

    println!("Listening on [{}] ...", socket);

    for stream in listener.incoming() {
        match stream {
            // Handle if an error
            Err(_) => (),

            // Handle if its a Result object
            Ok(mut stream) => {
                // Spawn a thread to handle the connection
                let counter = Arc::clone(&count);
                thread::spawn(move|| {
                    match stream.peer_addr() {
                        Err(_) => (),
                        Ok(pn) => println!("Received connection from: [{}]", pn),
                    }

                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();
                    
                    match str::from_utf8(&buf) {
                        Err(error) => {
                            println!("Received request error:\n{}", error);
                            return
                        },
                        Ok(body) => println!("Recieved request body:\n{}", body),
                    }

                    let body: &str = str::from_utf8(&buf).unwrap();
                    if body == "" {
                        return;
                    }

                    // Increase the value of the int the reference is pointing to.
                    let mut num = counter.lock().unwrap();
                    *num += 1;

                    // let path: &str = "original";
                    // stream.write(handle_request(path).as_bytes()).unwrap();
                    
                    let path: &str = extract_path(body);
                    handle_request(path, &mut stream);
                    println!("connection count: {}", num);
                    println!("Connection terminates.");
                });
            },
        }
    }

    drop(listener);
}
