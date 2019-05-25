// imports
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

// function to convert a C style string to a String
pub fn convert_c_string_to_String(raw_str_array: &[c_char]) -> String {
    // create the C string from the pure C string
    let raw_str = unsafe { 
        let ptr = raw_str_array.as_ptr();
        CStr::from_ptr(ptr)
    };

    // create an str from the C string
    let output = raw_str.to_str().expect("Failed to convert a raw string to a CString").to_owned();

    // return the String
    output
}