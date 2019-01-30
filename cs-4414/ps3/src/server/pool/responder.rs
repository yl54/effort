use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

use server::pool::handlers;
use server::pool::http::HRequest;

pub type Callback = fn(&mut HRequest);

#[derive(Clone)]
pub struct Handler {
    // path is the url path.
    pub path: String,

    // handler is the function to execute.
    handler: Callback,
}

// handler map struct
#[derive(Clone)]
pub struct HandlerMap {
    // handlers is a hashmap from the url path to handler functions.    
    handlers: HashMap<String, Handler>,
}

// responder struct
pub struct Responder {
    // handle to the running thread
    handle: thread::JoinHandle<String>,
}

// responder implementation
impl Responder {
    // new starts the responder thread that is continuously consuming requests
    // return the responder and its handle to the thread
    pub fn new(rx: Arc<Mutex<Receiver<HRequest>>>, handler: &Handler) -> Responder {
        let h = handler.clone();
        let h = thread::spawn(move || {
            loop {
                let mut h_request = rx.lock().unwrap().recv().unwrap();
                (h.handler)(&mut h_request);
            }
            "success".to_string()
        });

        Responder {
            handle: h,
        }
    }
}

