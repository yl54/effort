use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::http::{HRequest};

pub struct Parser {
    // tx_pipe is the Producer for asynchronous output.
    pub tx_pipe: Sender<HRequest>,
}

// Maybe the solution is to make our own Request object.
// or use the http request object
// Or figure out the lifetimes
impl Parser {
    pub fn new(tx: Sender<HRequest>) -> Parser {
        Parser{
            tx_pipe: tx,
        }
    }

    // parse parses the stream into a request and sends it off to the next step.
    pub fn parse(&self, mut stream: TcpStream) {
        // create the request
        // Read from the stream.
        // Q: Why do you have to read from the stream before stuff is written into it?
        let mut buf = [0 ;500];
        stream.read(&mut buf).unwrap();

        // Extract the body and path from the stream.
        let mut headers = [EMPTY_HEADER];
        let mut req = Request::new(&mut headers);
        let status = match req.parse(buf.as_ref()) {
            Ok(s) => {
            },
            Err(err) => {
                debug!("Failed parsing the bytes into a request.");
            },
        };

        // Convert to simple request
        let h = HRequest::convert(req, stream);

        // send it to the next place.
        self.tx_pipe.send(h);
    }
}

// TODO: Practice a small example to pass into mpsc
