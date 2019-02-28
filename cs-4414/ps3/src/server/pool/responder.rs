use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

use std::error::{Error as Erroror};
use std::io::{Error, ErrorKind};

use server::pool::handlers;
use server::pool::http::HRequest;

/* 
    Callback is a custom wrapper type for handler functions of HRequests.
    They are responsible for their own custom and for generating custom
      responses back to the client sending the http request.
*/
pub type Callback = fn(&mut HRequest);

/*
    Handler is a wrapper that associates the url path with the Callback dfunction
*/
#[derive(Clone)]
pub struct Handler {
    pub path: String,
    pub handler: Callback,
}

// HandlerMap registers each path with a Handler.
#[derive(Clone)]
pub struct HandlerMap {
    handlers: HashMap<String, Handler>,
}

/*
    Responders are workers that process the contents of the requests via custom
      handler functions. They recieve parsed requests from Parser workers. Each
      Responder worker is only responsible for handling requests for one url path.
*/
pub struct Responder {
    handle: thread::JoinHandle<String>,
}

impl Responder {
    // new starts the responder, which is continuously consuming parsed requests
    pub fn new(rx: Arc<Mutex<Receiver<Result<HRequest, Error>>>>, handler: &Handler) -> Responder {
        let h = handler.clone();
        let h = thread::spawn(move || {
            loop {
                let mut result = rx.lock().unwrap().recv().unwrap();
                match result {
                    Err(err) => {
                        debug!("Couldn't get the proper result: {}", err.description());
                        continue;
                    },
                    Ok(mut h_request) => { 
                        (h.handler)(&mut h_request); 

                        // TODO: Make handler return a Result<Response, Error>.
                        //       If response, write it to the stream. This can also send back custom errors as well.
                        //       If Error, then write a default error page with error description. This is for general errors.
                    }
                };
                
            }
            "success".to_string()
        });

        Responder {
            handle: h,
        }
    }
}

