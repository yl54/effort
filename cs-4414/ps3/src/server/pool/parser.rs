use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::error::{Error as Erroror};
use std::io::Error;

use httparse::{Error as HttpError, Request, EMPTY_HEADER};

use server::pool::http::{HRequest};
use server::pool::utils;

/* 
    Parsers are the initial workers to process http requests. They process http events in this order:
    * Parse the raw contents of the http response
    * Check if the path exists
        * If the path exists, pass it onto the next field
        * If the path doesn't exist, drop the request. (TODO: probably would be good to add a default 404 page)
*/
pub struct Parser {
    // thread handle
    handle: thread::JoinHandle<String>,
}

impl Parser {
    pub fn new(rx: Arc<Mutex<Receiver<Result<TcpStream, Error>>>>, tx_map: SenderMap) -> Parser {
        let h: thread::JoinHandle<String> = thread::spawn(move || {
            // Start infinite loop
            loop {
                // Get the lock for the reciever
                let res_stream: Result<TcpStream, Error> = rx.lock().unwrap().recv().unwrap();
                let mut stream = match res_stream {
                    Err(err) => {
                        debug!("Couldn't read the stream: {}", err.description());
                        continue;
                    },
                    Ok(mut stream) => { stream }
                };

                // Send to appropriate place in sender map
                let mut buf = [0 ;500];
                stream.read(&mut buf).unwrap();

                // Extract the body and path from the stream.
                let mut headers = [EMPTY_HEADER; utils::NUM_OF_HEADERS];
                let mut req = Request::new(&mut headers[..]);
                let status = match req.parse(buf.as_ref()) {
                    Ok(_s) => {
                    },
                    Err(err) => {
                        debug!("Failed parsing the bytes into a request: {}", err.description());
                        continue;
                    },
                };

                // Convert to simple request
                let h = HRequest::convert(req, stream);

                let path = match h.path.clone() {
                    Some(p) => p,
                    None => {
                        debug!("Failed getting the path from the request.");
                        continue;
                    },
                };
            
                match tx_map.map.get(&path.to_string()) {
                    Some(sender) => { 
                        sender.send(Ok(h));
                    },
                    None => {
                        debug!("No handler exists for this path: {}.", path);
                    },
                }
            }

            "Success".to_string()
        });

        Parser {
            handle: h,
        }
    }
}

/*
    SenderMaps exists as a bridge for Parsers to send parsed requests to Responders.
*/
#[derive(Clone)]
pub struct SenderMap {
    // hashmap from string to sender
    map: HashMap<String, Sender<Result<HRequest, Error>>>
}

impl SenderMap {
    pub fn new() -> SenderMap {
        SenderMap {
            map: HashMap::new(),
        }
    }

    pub fn register_sender(&mut self, path: String, sender: Sender<Result<HRequest, Error>>) {
        self.map.insert(path, sender);
    }
}

