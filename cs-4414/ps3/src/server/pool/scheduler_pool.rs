// struct for responder pool coordinator
    // list of responder pools that exist

// impl for responder pool coordinator
    // new

    // add handler
    // includes handler, count of workers, reciever atomoic reference to clone
    // inside each create of a pool

// struct for responder pool
    // list of scheduler workers

// impl for responder pool
    // new 
    // includes handler, count of number of pool, reciever atomoic reference to clone

        // for loop over the count of schedulers in the pool
            // create a new scheduler worker
            // each one needs handler and async reciever

            // push worker onto scheduler pool list


// struct for responder worker
    // thread handle

// impl for responder worker
    // new 
    // includes handler wrapper and reciever 
        // start a thread
        // q: is it possible to clone the handler?
        // a: yes, it just needs to be in a wrapper struct

            // loop

                // run the recieved thing through the handler

        // return the scheduler worker with the thread attached to it


