use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::http::{HRequest};

pub struct Parser {
    // thread handle
    handle: thread::JoinHandle<String>,
}

impl Parser {
    // new
    // include a SenderMap and reciever
    pub fn new(rx: Arc<Mutex<Receiver<TcpStream>>>, tx_map: SenderMap) -> Parser {
        // start a thread
        let h: thread::JoinHandle<String> = thread::spawn(move || {
            // start infinite loop
            loop {
                // Get the lock for the reciever
                let mut stream = rx.lock().unwrap().recv().unwrap();

                // Send to appropriate place in sender map
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
                        sender.send(h);
                    },
                    None => {
                        debug!("No handler exists for this path: {}.", path);
                    },
                }
            }

            "Success".to_string()
        });

        // return the struct with the handle
        Parser {
            handle: h,
        }
    }
}

// struct sender map
#[derive(Clone)]
pub struct SenderMap {
    // hashmap from string to sender
    map: HashMap<String, Sender<HRequest>>
}

impl SenderMap {
    pub fn new() -> SenderMap {
        SenderMap {
            map: HashMap::new(),
        }
    }

    pub fn register_sender(&mut self, path: String, sender: Sender<HRequest>) {
        self.map.insert(path, sender);
    }
}

