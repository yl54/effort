use std::error::{Error as Erroror};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::io::Error;

use server::pool::http::HRequest;
use server::pool::responder::Callback;
use server::pool::parser_pool::ParserPool;
use server::pool::responder_pool::ResponderPoolCoordinator;

// Local server address and port
const SERVER_ADDR: &str = "127.0.0.1";
const SERVER_PORT: &str = "20001";

/*
    Webservers manage the workers that handle the process of parsing
    incoming requests and responding to the client.
*/
pub struct Webserver {
    l: TcpListener,
    pp: ParserPool,
    rpc: ResponderPoolCoordinator,
    tx: Sender<Result<TcpStream, Error>>,
}

impl Webserver {
    pub fn new(parser_count: usize) -> Webserver {
        // Create a tcp listener.
        let full_address = format!("{}{}{}", SERVER_ADDR, ":", SERVER_PORT);
        let listener = TcpListener::bind(full_address).expect("Could not bind to address.");

        // Create the channel from the initial listener to the parser pool.
        let (tx, rx): (Sender<Result<TcpStream, Error>>, Receiver<Result<TcpStream, Error>>) = mpsc::channel();
        let rx_arc = Arc::new(Mutex::new(rx));

        // Create the Webserver.
        Webserver {
            l: listener,
            pp: ParserPool::new(parser_count, rx_arc),
            rpc: ResponderPoolCoordinator::new(),
            tx: tx,
        }
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

        // Start consuming http requests.
        for stream in self.l.incoming() {
            self.tx.send(stream);
        }
    }
}
