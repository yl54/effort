use std::ffi::CString;
use std::ptr;

// import ash
use ash::{Entry, Instance};

use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;

use ash::vk::{ApplicationInfo, InstanceCreateFlags, InstanceCreateInfo, StructureType};

// function to init a vulkan instance
pub fn create_vk_instance_struct() -> Instance {
    // Create the app info
    let vk_app_info = create_vk_application_info_struct();

    // create the instance create info struct
    let instance_create_info = create_vk_instance_create_info_struct(&vk_app_info);

    // create an entry
    let entry = match Entry::new() {
        Ok(e) => e,
        Err(err) => {
            // TODO: Fail here, return the Err(), pick something to stop program from progressing maybe. Plenty of options
            panic!("Failed to create instance: {}", err);
        }
    };

    // create an instance
    // q: what is the parameter?
    let instance = unsafe {
        match entry.create_instance(&instance_create_info, None) {
            Ok(i) => i,
            Err(err) => {
                // TODO: Fail here, return the Err(), pick something to stop program from progressing maybe. Plenty of options
                panic!("Failed to create instance: {}", err);
            }
        }
    };

    instance
}

// function to get a create instance info struct
fn create_vk_instance_create_info_struct(app_info: &ApplicationInfo) -> InstanceCreateInfo {
    // create instance create flags
    let vk_instance_create_flags = InstanceCreateFlags::empty();

    // Determine extensions available for use.
    let mut enabled_extension_count = 0;
    let mut enabled_extension_names: Vec<String> = vec![];

    // Determine layers needed.
    let mut enabled_layer_count = 0;

    // q: do i need to specify which validation layers i need as well?

    // q: do i need to specify which extensions exist here as well?
    // maybe this check belongs as one of the validation layers

    // create the instance create info struct
    let vk_instance_create_info = InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk_instance_create_flags,
        p_application_info: app_info,
        enabled_layer_count: enabled_layer_count,
        pp_enabled_layer_names: ptr::null(),
        enabled_extension_count: enabled_extension_count,
        pp_enabled_extension_names: ptr::null(),
    };

    vk_instance_create_info
}

// function to create vulkan metadata
fn create_vk_application_info_struct() -> ApplicationInfo {
    let app_info: ApplicationInfo = ApplicationInfo {
        s_type: StructureType::APPLICATION_INFO,
        p_next: ptr::null(),
        p_application_name: CString::new("Application Name")
            .expect("Failed to create engine name CString.")
            .as_ptr(),
        application_version: 1,
        p_engine_name: CString::new("Engine Name")
            .expect("Failed to create engine name CString.")
            .as_ptr(),
        engine_version: 1,
        api_version: 1,
    };

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
