use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use server::utils;

// handle_default reads the path and gives a response. This is the default handler
pub fn handle_default(stream: &mut TcpStream) {
    // Pick the page based off of the request.
    let html_file_path: &str = "files/original.html";

    // Write to stream.
    write_stream(html_file_path, stream);
}

// handle_great gets the trash page and shows it to the user.
pub fn handle_great(stream: &mut TcpStream) {
    // Pick the page based off of the request.
    let html_file_path: &str = "files/great.html";

    // Write to stream.
    write_stream(html_file_path, stream);
}

// handle_trash gets the trash page and shows it to the user.
pub fn handle_trash(stream: &mut TcpStream) {
    // Pick the page based off of the request.
    let html_file_path: &str = "files/trash.html";

    // Write to stream.
    write_stream(html_file_path, stream);
}

// handle_write_stream gets the file from the path and writes it as a response.
fn write_stream(file_path: &str, stream: &mut TcpStream) {
    // Create the full http response based off of the page
    let html_file_contents: String = utils::get_file_contents(file_path);

    // Return the string of the http response
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}\r\n", html_file_contents);
    stream.write(response.as_bytes()).unwrap();
}
