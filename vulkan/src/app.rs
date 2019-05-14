// imports

use std::ffi::CString;
use std::ptr;

// import ash
use ash::vk::{
    ApplicationInfo,
    InstanceCreateInfo,
    StructureType
};

// cfg for debug mode

// app struct + fields
pub struct App {
    // vk instance?

    // vk physical device

    // vk pipelines?

    // vk command makers?
}

// app impl
impl App {
    // q: do we want this or just have outside call these two separately
    // a: easier to call separately from outside
    // function to run the program, public
        // create a vk instance

        // check if the vk instance was properly created

        // setup a debug messenger

        // setup a physical device

        // run loop with app logic

    
    // function to init a vulkan instance
        // Create the app info

        // create the instance create info struct

        // create an instance

    // function to create vulkan metadata
    fn get_vk_application_info_struct(&self) -> ApplicationInfo {
        // Get the v1 version of Application Info
        let app_info: ApplicationInfo = ApplicationInfo {
            // Structure type enum
            s_type: StructureType::APPLICATION_INFO,

            // pointer to extensions
            p_next: ptr::null(),

            // app name
            p_application_name: CString::new("Application Name").expect("Failed to create engine name CString.").as_ptr(),

            // app version
            application_version: 1,

            // engine name
            p_engine_name: CString::new("Engine Name").expect("Failed to create engine name CString.").as_ptr(),

            // engine version
            engine_version: 1,

            // api verison
            api_version: 1,
        };

        // enabled extension count

        // enabled extension names

        // number of layers to validate

        // q: do i need to specify which validation layers i need as well?

        // q: do i need to specify which extensions exist here as well?
        // maybe this check belongs as one of the validation layers

        // return metadata struct
        app_info
    }

    // function to setup all of the extensions necessary
        // get a list of all of the extensions supported here

        // make sure necessary extensions exist

    // function to check if an extension is supported by vulkan

     // q: does this need to be in a separate function?
    // function to init validation layers
        // check which validation layers exist?

        // get the list of validation layers to use?

        // use either debug mode or non debug mode

    // function to implement debug callback
    // vulkan has in house attachments for debug messengers
        // this is to output a message somewhere for debugging (i think)

    // function to setup physical device
    // the physical device refers to a graphics card
        // call api to check for available devices

        // check if any devices are ready for use
            // if not, then signal a quit or something (can be result failre)

        // get each graphics card properties in acquired list

        // go through each device
            // check if card supports functionality of what user wants to do or whatever

        // check if there are no suitable devices
            // if not, signal a quit or something 

    // function to check if device is suitable for application
    // fake function: return true
    // easy way: true/false
    // alternative granular control: attach a score to the device, pick high score
        // get the properties of the device

        // check for something?

        // get the features of the device

        // check for something?

        // return true if it passes everything

    // function to check for queue families that exist
    // different devices have support for different families
        // query for device queue family properties

    // function to run the main logic for the application
}

// app impl for drop trait
impl Drop for App {
    // main drop function
    fn drop(&mut self) {
        // delete debugger extension

        // delete vulkan instance
    }

    // function to delete vulkan instance

    // function to delete debugger extension
}