use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use httparse::{Request, EMPTY_HEADER};

use server::simple::utils;

pub type Callback = fn(TcpStream);

/*
    Handlers are wrappers for Callback functions. It includes relevant metadata.
*/
struct Handler {
    // path is the url path.
    path: String,

    // handler is the function to execute.
    handler: Callback,

    // count is the number of times this handler has been requested.
    count: Arc<Mutex<u16>>,
}

/*
    Schedulers parse and respond to http requests.
*/
pub struct Scheduler {
    // handlers is a hashmap from the url path to handler functions.    
    handlers: HashMap<String, Handler>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            handlers: HashMap::new(),
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
    }

    // schedule_stream gets a request from the input stream and writes to it.
    pub fn schedule_stream(&mut self, mut stream: TcpStream) {
        // Read from the stream.
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
                return;
            },
        };
    
        match self.handlers.get(&path.to_string()) {
            Some(h) => { 
                (h.handler)(stream);
                let mut c = h.count.lock().unwrap();
                *c += 1;
            },
            None => {},
        }
    }
}
