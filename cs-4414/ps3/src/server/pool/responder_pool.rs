use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

use std::error::{Error as Erroror};
use std::io::Error;

use server::pool::http::HRequest;
use server::pool::responder::{ Callback, Handler, Responder };

/*
    ResponderPoolCoordinators coordinate the existence of the ResponderPools.
*/
pub struct ResponderPoolCoordinator{
    pools: HashMap<String, ResponderPool>,
}

impl ResponderPoolCoordinator {
    pub fn new() -> ResponderPoolCoordinator {
        ResponderPoolCoordinator {
            pools: HashMap::new(),
        }
    }

    // add_pool adds a new ResponderPool to the coordinator.
    pub fn add_pool(&mut self, path: String, c: Callback, rx: Arc<Mutex<Receiver<Result<HRequest, Error>>>>, count: usize) {
        let handler = Handler {
            path: path.clone(),
            handler: c, 
        };
        let path = handler.path.clone();
        let r_pool = ResponderPool::new(rx, handler, count);
        self.pools.insert(path, r_pool);
    }

    // run starts all of the ResponderPools.
    pub fn run(&mut self) {
        for (_path, mut pool) in &mut self.pools {
            pool.run();
        }
    }
}

/*
    ResponderPools manage and record the existence of individual Responders.
    Each ResponderPool is only responsible for handling the requests of one
    url path.
*/
struct ResponderPool {
    // pool contains all of the Responder workers in this pool.
    pool: Vec<Responder>,

    // rx is a reciever for events of a particular path.
    rx: Arc<Mutex<Receiver<Result<HRequest, Error>>>>,

    // h is the function that will be executed by the Responder on the HRequest.
    h: Handler,

    // count is the size of the pool.
    // TODO: Get a better name?
    count: usize,
}

impl ResponderPool {
    pub fn new(rx: Arc<Mutex<Receiver<Result<HRequest, Error>>>>, handler: Handler, count: usize) -> ResponderPool {
        ResponderPool {
            pool: vec![],
            rx: rx,
            h: handler,
            count: count,
        }
    }

    // run starts all of the workers.
    pub fn run(&mut self) {
        for _x in 0..self.count {
            let responder = Responder::new(self.rx.clone(), &self.h);
            self.pool.push(responder);
        }
    }
}
