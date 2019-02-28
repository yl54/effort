// This file contains a webserver.

use std::collections::HashMap;
use std::error::{Error as Erroror};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::io::Error;

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::handlers;
use server::pool::http::HRequest;
use server::pool::responder::Callback;
use server::pool::parser_pool::ParserPool;
use server::pool::responder_pool::ResponderPoolCoordinator;
use server::pool::utils;

// Local server address and port
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

pub struct Webserver {
    l: TcpListener,
    pp: ParserPool,
    rpc: ResponderPoolCoordinator,

    // req_total is the count of how many total requests have been recieved.
    req_total: Arc<Mutex<u16>>,

    // initial sender to parser pool
    tx: Sender<Result<TcpStream, Error>>,
}

impl Webserver {
    // New function
    pub fn new(parser_count: usize) -> Webserver {
        // Create a tcp listener.
        // Bind the listener to some address. Use the local address for now.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");

        // Create the channel from the initial listener to the parser pool
        let (tx, rx): (Sender<Result<TcpStream, Error>>, Receiver<Result<TcpStream, Error>>) = mpsc::channel();
        let rx_arc = Arc::new(Mutex::new(rx));
        return Webserver {
            l: listener,
            req_total: Arc::new(Mutex::new(0)),
            pp: ParserPool::new(parser_count, rx_arc),
            rpc: ResponderPoolCoordinator::new(),
            tx: tx,
        };
    }

    // register_handler registers a handler with a path
    pub fn register_handler(&mut self, path: String, handler: Callback, responder_pool_count: usize) {
        let path_cl = path.clone();

        // Create a channel from the parser pool to the responder pool
        let (tx, rx): (Sender<Result<HRequest, Error>>, Receiver<Result<HRequest, Error>>) = mpsc::channel();
        let rx_arc = Arc::new(Mutex::new(rx));

        // Register with the parser pool and the responder pool coordinator
        self.pp.register_parser(path, tx);
        self.rpc.add_pool(path_cl, handler, rx_arc, responder_pool_count);
    }

    // listen listens for requests and executes the proper handler if possible.
    pub fn listen(&mut self) {
        // Start the parser pool and responder pool coordinator
        self.rpc.run();
        self.pp.run();

        for stream in self.l.incoming() {
            // Send the input to parser pool
            self.tx.send(stream);
        }
    }
}
