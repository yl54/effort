use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::handlers;
use server::pool::http::HRequest;
use server::pool::utils;

pub type Callback = fn(&mut HRequest);

// This works
#[derive(Clone)]
struct g {
    c: Callback,
}

struct Handler {
    // path is the url path.
    path: String,

    // handler is the function to execute.
    handler: Callback,

    // count is the number of times this handler has been requested.
    count: Arc<Mutex<u16>>,
}

// Schedulers get TCP connections and schedules handling each request.
pub struct Scheduler {
    // handlers is a hashmap from the url path to handler functions.    
    handlers: HashMap<String, Handler>,

    // rx is a reciever for parsed request
    //rx: Arc<Mutex<Receiver<HRequest>>>,

    //
    handle: thread::JoinHandle<String>,
}

impl Scheduler {
    pub fn new(rx: Arc<Mutex<Receiver<HRequest>>>) -> Scheduler {
        let h = thread::spawn(move || {
            loop {
                let hRequest = rx.lock().unwrap().recv().unwrap();

                let path = match &hRequest.path {
                    Some(p) => p.clone(),
                    None => "".to_string(),
                };
                /*
                match self.handlers.get(&path.to_string()) {
                    Some(h) => { 
                        (h.handler)(hRequest);
                        let mut c = h.count.lock().unwrap();
                        *c += 1;
                    },
                    None => {
                        handlers::handle_default(hRequest); 
                    },
                }
                */
                
            }
            "success".to_string()
        });

        Scheduler {
            handlers: HashMap::new(),
            //rx: rx,
            handle: h,
        }
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
        
        /*

        This compiles
        let g = g {
            c: handler,
        };

        let g_cl = g.clone();
        */
    }

    // schedule_stream gets a request from the input stream and writes to it.
    pub fn schedule_stream(&mut self, mut stream: TcpStream) {
        // Read from the stream.
        // Q: Why do you have to read from the stream before stuff is written into it?
        let mut buf = [0 ;500];
        stream.read(&mut buf).unwrap();

        // Extract the body and path from the stream.
        let mut headers = [EMPTY_HEADER; utils::NUM_OF_HEADERS];
        let mut req = Request::new(&mut headers[..]);
        let status = match req.parse(buf.as_ref()) {
            Ok(s) => s,
            Err(err) => {
                debug!("Failed parsing the bytes into a request.");
                return
            },
        };

        let path = match req.path {
            Some(p) => p,
            None => {
                debug!("Failed getting the path from the request.");
                return
            },
        };

        let hRequest = HRequest::convert(req, stream);
    
        match self.handlers.get(&path.to_string()) {
            Some(h) => { 
                //(h.handler)(hRequest);
                let mut c = h.count.lock().unwrap();
                *c += 1;
            },
            None => {
                //handlers::handle_default(hRequest); 
            },
        }
    }

    /*
    // TODO: Add async reciever support
    // Look here: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
    pub fn schedule_requests(&mut self) {
        let h = thread::spawn(move || {
            loop {
                /*
                match self.rx.lock().unwrap().recv().unwrap() {
                    
                    Some(hRequest) => {
                        let path = match &hRequest.path {
                            Some(p) => p.clone(),
                            None => "".to_string(),
                        };
                        match self.handlers.get(&path.to_string()) {
                            Some(h) => { 
                                (h.handler)(hRequest);
                                let mut c = h.count.lock().unwrap();
                                *c += 1;
                            },
                            None => {
                                handlers::handle_default(hRequest); 
                            },
                        }
                    }
                    None => {}
                }
                */

                let hRequest = self.rx.lock().unwrap().recv().unwrap();

                let path = match &hRequest.path {
                    Some(p) => p.clone(),
                    None => "".to_string(),
                };

                match self.handlers.get(&path.to_string()) {
                    Some(h) => { 
                        //(h.handler)(hRequest);
                        let mut c = h.count.lock().unwrap();
                        *c += 1;
                    },
                    None => {
                        //handlers::handle_default(hRequest); 
                    },
                }
                
            }
            "success".to_string()
        });

        self.handle_list.push(h);
    }
    */
}
