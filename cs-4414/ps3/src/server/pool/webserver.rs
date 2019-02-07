// This file contains a webserver.

use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::handlers;
use server::pool::http::HRequest;
use server::pool::parser_pool::ParserPool;
use server::pool::responder_pool::ResponderPoolCoordinator;
use server::pool::scheduler::{Callback, Scheduler};
use server::pool::utils;

// Server address
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

// Webserver 
pub struct Webserver {
    // l is the handle to the request listener.
    l: TcpListener,

    // sc is a scheduler for each connection that comes to the Webserver.    
    //sc: Scheduler,

    // parser pool
    pp: ParserPool,

    // responder pool
    rpc: ResponderPoolCoordinator,

    // req_total is the count of how many total requests have been recieved.
    req_total: Arc<Mutex<u16>>,

    // initial sender to parser
    tx: Sender<TcpStream>,
}

impl Webserver {
    // New function
    pub fn new() -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");

        let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();

        let rx_arc = Arc::new(Mutex::new(rx));
        let rx_cl = Arc::clone(&rx_arc);

        let parser_count = 5;

        return Webserver {
            l: listener,
            //p: Parser::new(tx),
            //sc: Scheduler::new(rx_cl),
            req_total: Arc::new(Mutex::new(0)),
            pp: ParserPool::new(parser_count, rx_arc),
            rpc: ResponderPoolCoordinator::new(),
            tx: tx,
        };
    }

    // register_handler registers a handler with a path.
    pub fn register_handler(&mut self, path: String, handler: Callback) {
        //self.sc.register_handler(path, handler);

        // Create a sender/reciever for that path
        let (tx, rx): (Sender<HRequest>, Receiver<HRequest>) = mpsc::channel();

        let rx_arc = Arc::new(Mutex::new(rx));
        let rx_cl = Arc::clone(&rx_arc);

        let path_cl = path.clone();
        let tx_cl = tx.clone();

        let responder_pool_count = 10;

        // Register in parser pool
        self.pp.register_parser(path_cl, tx_cl);

        // Register in responder pool
        self.rpc.add_pool(path.clone(), handler, rx_arc, responder_pool_count);
    }

    // listen listens for requests and executes the proper handler if possible.
    pub fn listen(&mut self) {
        // Start the scheduler listeners
        // self.sc.schedule_requests();
        self.rpc.run();
        self.pp.run();


        // Start the listener infinite loop.
        for stream in self.l.incoming() {
            // Increase the value of the int the reference is pointing to.
            let mut num_total = self.req_total.lock().unwrap();
            *num_total += 1;

            match stream {
                Err(err) => debug!("Couldn't read the stream: {}", err.description()),
                Ok(mut stream) => {
                    // self.sc.schedule_stream(stream);
                    self.tx.send(stream);

                    println!("sent a message");
                }
            }
        }
    }
}
