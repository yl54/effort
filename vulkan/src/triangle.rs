// imports

// import ash

// app struct + fields
    // vk instance?

    // vk pipelines?

    // vk command makers?

// app impl
    // q: do we want this or just have outside call these two separately
    // a: easier to call separately from outside
    // function to run the program, public
        // create a vk instance

        // check if the vk instance was properly created

        // run loop with app logic

    // function to init a vulkan instance
        // Create the metadata struct thing

        // create an instance 

    // function to create vulkan metadata 
        // stype
        // q: why is this needed if its always a specific struct?

        // application name

        // application version

        // engine name

        // engine version

        // api version

        // enabled extension count

        // enabled extension names

        // number of layers to validate

        // q: do i need to specify which validation layers i need as well?

        // q: do i need to specify which extensions exist here as well?
        // maybe this check belongs as one of the validation layers

        // return metadata struct

    // function to check if an extension is supported by vulkan

    // function to run the main logic for the application

    // q: does this need to be in a separate function?
    // function to init validation layers
        // check which validation layers exist?

        // get the list of validation layers to use?


// app impl for drop trait
    // main drop function

    // function to delete vulkan instance

    // function to destroy a window
