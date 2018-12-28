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

// extract_path extracts the path from a HTTP POST request.
pub fn extract_path(request: &str) -> &str {
    // Split the string by new line
    let lines: Vec<&str> = request.split("\n").collect();

    // Check if the vec is valid.
    if lines.len() <= 0 {
        return "";
    }

    // Take the first string
    let line: &str = lines[0];

    // Split the string by " "
    let spl: Vec<&str> = line.split(" ").collect();
    println!("{:?}", spl);

    // Check if the vec is valid.
    if spl.len() <= 0 {
        return "";
    }

    // Check the length
    if spl.len() != 3 {
        println!("part does not match format");
        return "";
    }

    // Get the second string
    // take the &str in the middle, ignore the first character
    let part: &str = &spl[1][1..];
    println!("part: {}", part);

    // Try to match and return.
    return part;
}
