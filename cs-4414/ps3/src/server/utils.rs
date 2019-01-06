use std::fs::File;
use std::io::{Read, Write};

// get_file_contents attempts to get the contents of the page.
pub fn get_file_contents(file_path: &str) -> String {
    // Attempt to open the file.
    let mut f = File::open(file_path).expect("file not found");

    // Allocate a new string, which is dynamic.
    let mut contents = String::new();

    // Read the file into the string
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Return the string object.
    return contents;
}

fn debug_vec_str(lines: Vec<&str>) {
    for l in &lines {
        debug!("{}", l);
    }
}
