use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::io::Error;

use server::pool::http::{HRequest};
use server::pool::parser::{Parser, SenderMap};

/*
    ParserPools manage and record the existence of Parser workers.
*/
pub struct ParserPool {
    // count is the size of the pool.
    count: usize,

    // pool exists to hold a reference to all of the Parser workers.
    pool: Vec<Parser>,  

    // rx is a reciever for raw TcpStreams coming from the Webserver.
    // Each Parser worker will consume streams from the Webserver via this channel.
    rx: Arc<Mutex<Receiver<Result<TcpStream, Error>>>>,

    // path_map is the SenderMap that every Parser will use to refer parsed requests to.
    path_map: SenderMap,
}

impl ParserPool {
    pub fn new(count: usize, rx: Arc<Mutex<Receiver<Result<TcpStream, Error>>>>) -> ParserPool {
        ParserPool {
            count: count, 
            pool: vec![],
            rx: rx,
            path_map: SenderMap::new(),
        } 
    }

    // register_handler registers the Sender with its respective path.
    pub fn register_parser(&mut self, path: String, sender: Sender<Result<HRequest, Error>>) {
        self.path_map.register_sender(path, sender);
    }

    // run starts all of the Parser workers. 
    pub fn run(&mut self) {
        for _x in 0..self.count {
            let rx_cl = self.rx.clone();
            let path_map_cl = self.path_map.clone();
            let parser = Parser::new(rx_cl, path_map_cl);
            self.pool.push(parser);
        }
    }
}