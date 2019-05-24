// imports

use std::ffi::CString;
use std::ptr;

// import ash
use ash::{
    Entry,
    Instance
};

use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;

use ash::vk::{
    ApplicationInfo,
    InstanceCreateFlags,
    InstanceCreateInfo,
    StructureType
};

use crate::app_vk;


// cfg for debug mode

// app struct + fields
pub struct App {
    // entry
    entry: Entry,

    // vk instance?
    instance: Instance,

    // vk physical device

    // vk pipelines?

    // vk command makers?
}

// app impl
impl App {
    // function to create a new App
    pub fn new() -> App {
        // create an entry
        // q: can i use multiple of these, or can I only have one in play at any given moment?
        // a: not sure, but stick with 1 for now.
        let entry = match Entry::new() {
            Ok(e) => e,
            Err(err) => {
                // TODO: Fail here, return the Err(), pick something to stop program from progressing maybe. Plenty of options
                panic!("Failed to create instance: {}", err);
            }
        };

        // create a vk instance
        let instance = app_vk::create_vk_instance_struct(&entry);

        // check if the vk instance was properly created

        // setup a debug messenger

        // setup a physical device

        App {
            entry: entry,
            instance: instance
        }
    }

    // q: do we want this or just have outside call these two separately
    // a: easier to call separately from outside
    // function to run the program, public
        // create a vk instance

        // check if the vk instance was properly created

        // setup a debug messenger

        // setup a physical device

        // run loop with app logic

   
    // function to run the main logic for the application
}

// app impl for drop trait
impl Drop for App {
    // main drop function
    fn drop(&mut self) {
        // delete debugger extension

        // delete vulkan instance
        // q: what is the parameter?
        unsafe {
            self.instance.destroy_instance(None);
        };
    }
}