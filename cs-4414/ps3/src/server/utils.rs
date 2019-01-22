use std::borrow::Borrow;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::result::Result;

use http::header::{self, HeaderName, HeaderValue};
use http::{Response, StatusCode};
use httparse::{Request, Status, EMPTY_HEADER};

pub const NUM_OF_HEADERS: usize = 30;

// get_file_contents attempts to get the contents of the page.
pub fn get_file_contents(file_path: &str) -> String {
    // Attempt to open the file.
    let mut f = File::open(file_path).expect("file not found");

    // Allocate a new string, which is dynamic.
    let mut contents = String::new();

    // Read the file into the string
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Return the string object.
    return contents;
}

fn debug_vec_str(lines: Vec<&str>) {
    for l in &lines {
        debug!("{}", l);
    }
}

// write_response writes an http response to a stream.
pub fn write_response<T: Borrow<[u8]>>(response: Response<T>, mut stream: TcpStream) {
    // Get the parts of the http response.
    let (parts, body) = response.into_parts();
    let body: &[u8] = body.borrow();

    // Get the initial header text.
    let mut text = format!(
        "HTTP/1.1 {} {}\r",
        parts.status.as_str(),
        parts.status.canonical_reason()
                    .expect("Unsupported HTTP Status"),
    );

    // Get the headers and write to text.
    for (key, value) in parts.headers.iter() {
        text = format!("{}\r\n{}: {}", text, key.as_str(), value.to_str().unwrap());
    }
    text = format!("{}\r\n\n", text);

    // Write to stream.
    stream.write(text.as_bytes());
    stream.write(body);

    // Q: what is stream flush?
    stream.flush();
}