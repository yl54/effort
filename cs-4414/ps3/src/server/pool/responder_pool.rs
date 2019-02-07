use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use server::pool::http::HRequest;
use server::pool::responder::{ Callback, Handler, Responder };

// struct for responder pool coordinator
pub struct ResponderPoolCoordinator{
    // list of responder pools that exist
    pools: HashMap<String, ResponderPool>,
}

// impl for responder pool coordinator
impl ResponderPoolCoordinator {
    // new
    pub fn new() -> ResponderPoolCoordinator {
        ResponderPoolCoordinator {
            pools: HashMap::new(),
        }
    }

    // add handler
    pub fn add_pool(&mut self, path: String, c: Callback, rx: Arc<Mutex<Receiver<HRequest>>>, count: usize) {
        let handler = Handler {
            path: path.clone(),
            handler: c, 
        };
        let path = handler.path.clone();
        let r_pool = ResponderPool::new(rx, handler, count);
        self.pools.insert(path, r_pool);
    }

    // run
    pub fn run(&mut self) {
        for (path, mut pool) in &mut self.pools {
            pool.run();
        }
    }
}

// struct for responder pool
struct ResponderPool {
    // list of scheduler workers
    pool: Vec<Responder>,

    // A reference to the reciever. This is shared by all of the responder instances.
    rx: Arc<Mutex<Receiver<HRequest>>>,

    // Handler
    h: Handler,

    // count
    count: usize,
}

// impl for responder pool
impl ResponderPool {
    // new 
    // includes handler, count of number of pool, reciever atomoic reference to clone
    pub fn new(rx: Arc<Mutex<Receiver<HRequest>>>, handler: Handler, count: usize) -> ResponderPool {
        ResponderPool {
            pool: vec![],
            rx: rx,
            h: handler,
            count: count,
        }
    }

    pub fn run(&mut self) {
        // for loop over the count of schedulers in the pool
        for x in 0..self.count {
            // create a new scheduler worker            
            let responder = Responder::new(self.rx.clone(), &self.h);

            // push worker onto scheduler pool list
            self.pool.push(responder);
            println!("made a responder");
        }
    }
}
