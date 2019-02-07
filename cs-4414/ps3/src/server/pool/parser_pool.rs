use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use server::pool::http::{HRequest};
use server::pool::parser::{Parser, SenderMap};

// struct for parser pool
pub struct ParserPool {
    // count
    count: usize,

    // vec to parsers
    pool: Vec<Parser>,  

    // reciever for thing
    rx: Arc<Mutex<Receiver<TcpStream>>>,

    // Map to send requests
    path_map: SenderMap,
}

// impl for parser pool
impl ParserPool {
    // new
    // include count, reciever
    pub fn new(count: usize, rx: Arc<Mutex<Receiver<TcpStream>>>) -> ParserPool {
        // return new
        ParserPool {
            count: count, 
            pool: vec![],
            rx: rx,
            path_map: SenderMap::new(),
        } 
    }

    // register handler
    pub fn register_parser(&mut self, path: String, sender: Sender<HRequest>) {
        // add to the hashmap
        self.path_map.register_sender(path, sender);
    }

    // run 
    pub fn run(&mut self) {
        for x in 0..self.count {
            let rx_cl = self.rx.clone();
            let path_map_cl = self.path_map.clone();

            let parser = Parser::new(rx_cl, path_map_cl);

            // push worker onto parser pool list
            self.pool.push(parser);
        }
    }
}