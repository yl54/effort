// router struct
    // reciever struct

// router impl
    // new

        // clone the map from path to sender

        // run a thread
            
            // run a infinite loop

                // check if the path matches anywhere
                    // send to that channel

                // send to the default handler's channel

// router pool
    // router objects

    // hashmap from path to sender channel

    // count

// router impl
    // new
        // create the router

    // add to router 
    // includes the path, sender channel
        // add to the hashmap from path to sender

    // run the workers
        // for loop to count
            // Create a router

            // push router to pool
