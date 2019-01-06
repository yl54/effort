use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use http::header::{self, HeaderName, HeaderValue};
use http::{Response, StatusCode};
use regex::Regex;

use gash::executor::Executor;
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

// write_stream gets the file from the path and writes it as a response.
fn write_stream(file_path: &str, stream: &mut TcpStream) {
    // Create the full http response based off of the page
    let html_file_contents: String = utils::get_file_contents(file_path);

    // Create a response and attempt to write back to the stream.
    let mut response = Response::builder();
    match response.status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .header("charset", "UTF-8")
            .body(html_file_contents.as_bytes()) {
        Ok(res) => { utils::write_response(res, stream); },
        Err(err) => { debug!("Failed to create response: {}", err.description()); }
    }
}

// handle_utility_date gets the utility date page and shows it to the user.
pub fn handle_utility_date(stream: &mut TcpStream) {
    // Pick the page based off of the request.
    let path: &str = "files/utility/date.shtml";

    // Write to stream.
    write_utility_stream(path, stream);
}

// write_utility_stream handles paths through `utility`.
// These return dynamic shtml pages, usually running some shell command.
fn write_utility_stream(path: &str, stream: &mut TcpStream) {
    // Get the file handle.
    let file = File::open(path).expect("file not found");

    // Create a regex to search for.
    // Look for: `<!--#exec cmd="date" -->`
    let re = Regex::new(r#"<!--#exec cmd="(.*)" -->"#).expect("Creating regex failed.");

    // Create a new string with the new contents.
    let mut output_content: String = String::new();

    // Read each line from the file.
    for line in BufReader::new(file).lines() {
        let l = line.expect("line not found");        
        let mut output_line: String = l.clone();
        debug!("l: {}", l);

        // Check if the line matches the regex.
        let output_line = match re.captures(&l) {
            Some(cap) => {
                debug!("captured l: {}", l);

                // Read the command from the capture.
                let cmd = cap.get(1).unwrap().as_str().to_string();
                debug!("cmd: {}", cmd.clone());

                // Initialize the executor.
                let mut ex = Executor::new_without_sender();
                ex.set_current_cmd(cmd.clone());

                // Execute the command and get the output.
                let gash_output = ex.run_cmd();
                debug!("gash_output: {}", gash_output.clone());

                // Replace the shtml command with the output.
                re.replace(&output_line, gash_output.as_str()).to_string()
            },
            None => output_line,
        };

        // Add to the new string.
        output_content = format!("{}\n{}", output_content, output_line);
    }

    // Create a response and attempt to write back to the stream.
    let mut response = Response::builder();
    match response.status(StatusCode::OK)
                  .header("Content-Type", "text/html")
                  .header("charset", "UTF-8")
                  .body(output_content.as_bytes()) {
        Ok(res) => { utils::write_response(res, stream); },
        Err(err) => { debug!("Failed to create response: {}", err.description()); }
    }
}